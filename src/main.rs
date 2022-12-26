use std::fs::read_to_string;
use std::path::Path;
use std::{collections::HashMap, process};
use std::io::{self, Write};

#[allow(non_upper_case_globals)]
const flush: fn() = || io::stdout().flush().unwrap();

#[derive(Debug, Clone)]
enum Data {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
}

#[allow(clippy::inherent_to_string)]
impl Data {
    fn to_string(&self) -> String {
        match self {
            Data::Int(i) => i.to_string(),
            Data::Float(f) => f.to_string(),
            Data::String(s) => s.to_string().replace("\\n", "\n"),
            Data::Bool(b) => b.to_string(),
            Data::Null => "null".to_string(),
        }
    }
}

type FunctionBody = Vec<String>;

fn split_whitespace_not_in_quotes(s: &str) -> Vec<String> {
    let mut split = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for c in s.chars() {
        if c == '\"' {
            in_quotes = !in_quotes;
        }
        if c == ' ' && !in_quotes {
            split.push(current);
            current = String::new();
        } else {
            current.push(c);
        }
    }

    split.push(current);

    split
}

#[derive(Debug)]
struct RunTime {
    stack: Vec<Data>,
    registers: HashMap<String, Data>,
    functions: HashMap<String, FunctionBody>,
    equal_flag: bool,
    greater_flag: bool,
    lesser_flag: bool,
}

impl RunTime {
    fn new(code: String) -> RunTime {
        let mut functions = HashMap::new();
        let mut registers: HashMap<String, Data> = HashMap::new();

        //initialize registers
        for i in 0..=12 {
            registers.insert(format!("r{}", i), Data::Null); //general purpose registers rX
            registers.insert(format!("p{}", i), Data::Null); //parameter registers pX
            registers.insert(format!("ret{}", i), Data::Null); //return registers (for functions) retX
        }

        //find all the functions
        let mut lines = code.lines().filter(|l| !l.is_empty());

        while let Some(line) = lines.next() {
            if line.starts_with("fun ") {
                let split = line.split_whitespace().collect::<Vec<&str>>();

                if split.len() != 2 {
                    eprintln!("Invalid function declaration: {}", line);
                    process::exit(1);
                }

                let name = split[1].to_string();
                let mut body: Vec<String> = Vec::new();
                for l in lines.by_ref() {
                    if l.starts_with("end") {
                        break;
                    }
                    if l.trim_start().starts_with("//") || l.is_empty() { //ignore comments and empty lines
                        continue;
                    }
                    body.push(l.trim_start().to_string());
                }
                functions.insert(name, body);
            }
        }

        RunTime {
            stack: Vec::new(),
            registers,
            functions,
            equal_flag: false,
            greater_flag: false,
            lesser_flag: false,
        }
    }

    fn is_register(&self, reg: &String) -> bool {
        self.registers.contains_key(reg)
    }

    fn determine_type(&self, data: &String) -> Data {
        if data.starts_with('\"') && data.ends_with('\"') {
            Data::String(data[1..data.len() - 1].to_string())
        } else if data == "true" || data == "false" {
            Data::Bool(data == "true")
        } else if data.contains('.') {
            Data::Float(data.parse::<f64>().unwrap())
        } else if data.parse::<i64>().is_ok() {
            Data::Int(data.parse::<i64>().unwrap())
        }
        else {
            eprintln!("Unknown data type: {}", data);
            process::exit(1);
        }
    }

    fn execute_line(&mut self, line: &str) {
        let split = split_whitespace_not_in_quotes(line);
        if split.len() < 2 {
            eprintln!("Invalid instruction: {}", line);
            return;
        }

        let first_arg = split[1].to_string().replace(',', "");


        match split[0].trim() {
            "mv" => self.mov(first_arg, split[2].to_string()),
            "push" => self.push(first_arg),
            "pop" => self.pop(first_arg),
            "inc" => self.inc(&first_arg),
            "dec" => self.dec(&first_arg),
            "sub" => self.sub(&first_arg, &split[2].to_string()),
            "add" => self.add(&first_arg, &split[2].to_string()),
            "div" => self.div(&first_arg, &split[2].to_string()),
            "mul" => self.mul(&first_arg, &split[2].to_string()),
            "cmp" => self.cmp(&first_arg, &split[2].to_string()),
            "jmp" => self.jmp(&first_arg),
            "je" => self.je(&first_arg),
            "jne" => self.jne(&first_arg),
            "jg" => self.jg(&first_arg),
            "jge" => self.jge(&first_arg),
            "jl" => self.jl(&first_arg),
            "jle" => self.jle(&first_arg),

            _ => println!("Unknown command: {}", split[0]),
        }
    }

    fn push(&mut self, data: String) {
        if self.is_register(&data) {
            self.stack.push(self.registers.get(&data).unwrap().clone());
        } else {
            self.stack.push(self.determine_type(&data));
        }
    }

    fn pop(&mut self, out_reg: String) {
        if !self.is_register(&out_reg) {
            eprintln!("[pop] Attempted to pop into non-existant register: {}", out_reg);
            process::exit(1);
        }

        let data = self.stack.pop().unwrap();
        self.registers.insert(out_reg, data);
    }

    fn mov(&mut self, reg: String, data: String) {
        if !self.is_register(&reg) {
            eprintln!("[mov] Attempted to move data into a non-existant register: {}", reg);
            process::exit(1);
        }
        
        if self.is_register(&data) { //move register to register
            self.registers.insert(reg, self.registers.get(&data).unwrap().clone());
        }
        else { //move hardcoded data to register
            self.registers.insert(reg, self.determine_type(&data));
        }
    }

    fn inc(&mut self, reg: &String) {
        if !self.is_register(reg) {
            eprintln!("[inc] Attempted to increment non-existant register: {}", reg);
            process::exit(1);
        }

        let data = self.registers.get(reg).unwrap();
        match data {
            Data::Int(i) => self.registers.insert(reg.to_string(), Data::Int(i + 1)),
            Data::Float(f) => self.registers.insert(reg.to_string(), Data::Float(f + 1.0)),
            _ => {
                eprintln!("[inc] Attempted to increment non-numeric register: {}", reg);
                process::exit(1);
            }
        };
    }

    fn dec(&mut self, reg: &String) {
        if !self.is_register(reg) {
            eprintln!("[dec] Attempted to decrement non-existant register: {}", reg);
            process::exit(1);
        }

        let data = self.registers.get(reg).unwrap();
        match data {
            Data::Int(i) => self.registers.insert(reg.to_string(), Data::Int(i - 1)),
            Data::Float(f) => self.registers.insert(reg.to_string(), Data::Float(f - 1.0)),
            _ => {
                eprintln!("[dec] Attempted to decrement non-numeric register: {}", reg);
                process::exit(1);
            }
        };
    }

    fn add(&mut self, reg: &String, data: &String) {
        if !self.is_register(reg) {
            eprintln!("[add] Attempted to add to non-existant register: {}", reg);
            process::exit(1);
        }

        if self.is_register(data) {
            let reg_data = self.registers.get(reg).unwrap();
            let data_data = self.registers.get(data).unwrap();
            match reg_data {
                Data::Int(i) => {
                    match data_data {
                        Data::Int(j) => self.registers.insert(reg.to_string(), Data::Int(i + j)),
                        Data::Float(j) => self.registers.insert(reg.to_string(), Data::Float(*i as f64 + j)),
                        _ => {
                            eprintln!("[add] Attempted to add non-numeric data to register: {}", reg);
                            process::exit(1);
                        }
                    };
                }

                Data::Float(i) => {
                    match data_data {
                        Data::Int(j) => self.registers.insert(reg.to_string(), Data::Float(i + *j as f64)),
                        Data::Float(j) => self.registers.insert(reg.to_string(), Data::Float(i + j)),
                        _ => {
                            eprintln!("[add] Attempted to add non-numeric data to register: {}", reg);
                            process::exit(1);
                        }
                    };
                }

                Data::String(i) => {
                    match data_data {
                        Data::String(j) => self.registers.insert(reg.to_string(), Data::String(i.to_string() + j)),
                        _ => {
                            eprintln!("[add] Attempted to add non-string data to register: {}", reg);
                            process::exit(1);
                        }
                    };
                }

                _ => {
                    eprintln!("[add] Attempted to add to non-numeric / non-string register: {}", reg);
                    process::exit(1);
                }
            }
        }
        else {
            let reg_data = self.registers.get(reg).unwrap();
            let data_data = self.determine_type(data);
            match reg_data {
                Data::Int(i) => {
                    match data_data {
                        Data::Int(j) => self.registers.insert(reg.to_string(), Data::Int(i + j)),
                        Data::Float(j) => self.registers.insert(reg.to_string(), Data::Float(*i as f64 + j)),
                        _ => {
                            eprintln!("[add] Attempted to add non-numeric data to register: {}", reg);
                            process::exit(1);
                        }
                    };
                }

                Data::Float(i) => {
                    match data_data {
                        Data::Int(j) => self.registers.insert(reg.to_string(), Data::Float(i + j as f64)),
                        Data::Float(j) => self.registers.insert(reg.to_string(), Data::Float(i + j)),
                        _ => {
                            eprintln!("[add] Attempted to add non-numeric data to register: {}", reg);
                            process::exit(1);
                        }
                    };
                }

                _ => {
                    eprintln!("[add] Attempted to add to non-numeric register: {}", reg);
                    process::exit(1);
                }
            }
        }
    }

    fn sub(&mut self, reg: &String, data: &String) {
        if !self.is_register(reg) {
            eprintln!("[sub] Attempted to subtract from non-existant register: {}", reg);
            process::exit(1);
        }

        if self.is_register(data) {
            let reg_data = self.registers.get(reg).unwrap();
            let data_data = self.registers.get(data).unwrap();
            match reg_data {
                Data::Int(i) => {
                    match data_data {
                        Data::Int(j) => self.registers.insert(reg.to_string(), Data::Int(i - j)),
                        Data::Float(j) => self.registers.insert(reg.to_string(), Data::Float(*i as f64 - j)),
                        _ => {
                            eprintln!("[sub] Attempted to subtract non-numeric data from register: {}", reg);
                            process::exit(1);
                        }
                    };
                }

                Data::Float(i) => {
                    match data_data {
                        Data::Int(j) => self.registers.insert(reg.to_string(), Data::Float(i - *j as f64)),
                        Data::Float(j) => self.registers.insert(reg.to_string(), Data::Float(i - j)),
                        _ => {
                            eprintln!("[sub] Attempted to subtract non-numeric data from register: {}", reg);
                            process::exit(1);
                        }
                    };
                }

                _ => {
                    eprintln!("[sub] Attempted to subtract from non-numeric register: {}", reg);
                    process::exit(1);
                }
            }
        }
        else {
            let reg_data = self.registers.get(reg).unwrap();
            let data_data = self.determine_type(data);
            match reg_data {
                Data::Int(i) => {
                    match data_data {
                        Data::Int(j) => self.registers.insert(reg.to_string(), Data::Int(i - j)),
                        Data::Float(j) => self.registers.insert(reg.to_string(), Data::Float(*i as f64 - j)),
                        _ => {
                            eprintln!("[sub] Attempted to subtract non-numeric data from register: {}", reg);
                            process::exit(1);
                        }
                    };
                }

                Data::Float(i) => {
                    match data_data {
                        Data::Int(j) => self.registers.insert(reg.to_string(), Data::Float(i - j as f64)),
                        Data::Float(j) => self.registers.insert(reg.to_string(), Data::Float(i - j)),
                        _ => {
                            eprintln!("[sub] Attempted to subtract non-numeric data from register: {}", reg);
                            process::exit(1);
                        }
                    };
                }

                _ => {
                    eprintln!("[sub] Attempted to subtract from non-numeric register: {}", reg);
                    process::exit(1);
                }
            }
        }
    }

    fn div(&mut self, reg: &String, data: &String) {
        if !self.is_register(reg) {
            eprintln!("[div] Attempted to divide non-existant register: {}", reg);
            process::exit(1);
        }

        if self.is_register(data) {
            let reg_data = self.registers.get(reg).unwrap();
            let data_data = self.registers.get(data).unwrap();
            match reg_data {
                Data::Int(i) => {
                    match data_data {
                        Data::Int(j) => {
                            if *j == 0 {
                                eprintln!("[div] Attempted to divide by zero.");
                                process::exit(1);
                            }
                            else {
                                self.registers.insert(reg.to_string(), Data::Float(*i as f64 / *j as f64));
                            }
                        }
                        Data::Float(j) => {
                            if *j == 0.0 {
                                eprintln!("[div] Attempted to divide by zero.");
                                process::exit(1);
                            }
                            else {
                                self.registers.insert(reg.to_string(), Data::Float(*i as f64 / j));
                            }
                        }

                        _ => {
                            eprintln!("[div] Attempted to divide non-numeric data from register: {}", reg);
                            process::exit(1);
                        }
                    }
                }

                Data::Float(i) => {
                    match data_data {
                        Data::Int(j) => {
                            if *j == 0 {
                                eprintln!("[div] Attempted to divide by zero.");
                                process::exit(1);
                            }
                            else {
                                self.registers.insert(reg.to_string(), Data::Float(i / *j as f64));
                            }
                        }
                        Data::Float(j) => {
                            if *j == 0.0 {
                                eprintln!("[div] Attempted to divide by zero.");
                                process::exit(1);
                            }
                            else {
                                self.registers.insert(reg.to_string(), Data::Float(i / j));
                            }
                        }
                        _ => {
                            eprintln!("[div] Attempted to divide non-numeric data from register: {}", reg);
                            process::exit(1);
                        }
                    }
                }

                _ => {
                    eprintln!("[div] Attempted to divide from non-numeric register: {}", reg);
                    process::exit(1);
                }
            }
        }

        else {
            let reg_data = self.registers.get(reg).unwrap();
            let data_data = self.determine_type(data);
            match reg_data {
                Data::Int(i) => {
                    match data_data {
                        Data::Int(j) => {
                            if j == 0 {
                                eprintln!("[div] Attempted to divide by zero.");
                                process::exit(1);
                            }
                            else {
                                self.registers.insert(reg.to_string(), Data::Float(*i as f64 / j as f64));
                            }
                        }
                        Data::Float(j) => {
                            if j == 0.0 {
                                eprintln!("[div] Attempted to divide by zero.");
                                process::exit(1);
                            }
                            else {
                                self.registers.insert(reg.to_string(), Data::Float(*i as f64 / j));
                            }
                        }

                        _ => {
                            eprintln!("[div] Attempted to divide non-numeric data from register: {}", reg);
                            process::exit(1);
                        }
                    }
                }

                Data::Float(i) => {
                    match data_data {
                        Data::Int(j) => {
                            if j == 0 {
                                eprintln!("[div] Attempted to divide by zero.");
                                process::exit(1);
                            }
                            else {
                                self.registers.insert(reg.to_string(), Data::Float(i / j as f64));
                            }
                        }
                        Data::Float(j) => {
                            if j == 0.0 {
                                eprintln!("[div] Attempted to divide by zero.");
                                process::exit(1);
                            }
                            else {
                                self.registers.insert(reg.to_string(), Data::Float(i / j));
                            }
                        }
                        _ => {
                            eprintln!("[div] Attempted to divide non-numeric data from register: {}", reg);
                            process::exit(1);
                        }
                    }
                }

                _ => {
                    eprintln!("[div] Attempted to divide from non-numeric register: {}", reg);
                    process::exit(1);
                }
            }
        }
    }

    fn mul(&mut self, reg: &String, data: &String) {
        if !self.is_register(reg) {
            eprintln!("[mul] Attempted to multiply non-existant register: {}", reg);
            process::exit(1);
        }

        if self.is_register(data) {
            let reg_data = self.registers.get(reg).unwrap();
            let data_data = self.registers.get(data).unwrap();
            match reg_data {
                Data::Int(i) => {
                    match data_data {
                        Data::Int(j) => self.registers.insert(reg.to_string(), Data::Int(i * j)),
                        Data::Float(j) => self.registers.insert(reg.to_string(), Data::Float(*i as f64 * j)),
                        _ => {
                            eprintln!("[mul] Attempted to multiply non-numeric data from register: {}", reg);
                            process::exit(1);
                        }
                    };
                }

                Data::Float(i) => {
                    match data_data {
                        Data::Int(j) => self.registers.insert(reg.to_string(), Data::Float(i * *j as f64)),
                        Data::Float(j) => self.registers.insert(reg.to_string(), Data::Float(i * j)),
                        _ => {
                            eprintln!("[mul] Attempted to multiply non-numeric data from register: {}", reg);
                            process::exit(1);
                        }
                    };
                }

                _ => {
                    eprintln!("[mul] Attempted to multiply from non-numeric register: {}", reg);
                    process::exit(1);
                }
            }
        }

        else {
            let reg_data = self.registers.get(reg).unwrap();
            let data_data = self.determine_type(data);
            match reg_data {
                Data::Int(i) => {
                    match data_data {
                        Data::Int(j) => self.registers.insert(reg.to_string(), Data::Int(i * j)),
                        Data::Float(j) => self.registers.insert(reg.to_string(), Data::Float(*i as f64 * j)),
                        _ => {
                            eprintln!("[mul] Attempted to multiply non-numeric data from register: {}", reg);
                            process::exit(1);
                        }
                    };
                }

                Data::Float(i) => {
                    match data_data {
                        Data::Int(j) => self.registers.insert(reg.to_string(), Data::Float(i * j as f64)),
                        Data::Float(j) => self.registers.insert(reg.to_string(), Data::Float(i * j)),
                        _ => {
                            eprintln!("[mul] Attempted to multiply non-numeric data from register: {}", reg);
                            process::exit(1);
                        }
                    };
                }

                _ => {
                    eprintln!("[mul] Attempted to multiply from non-numeric register: {}", reg);
                    process::exit(1);
                }
            }
        }
    }

    fn cmp(&mut self, reg: &String, data: &String) {
        if !self.is_register(reg) {
            eprintln!("[cmp] Attempted to compare non-existant register: {}", reg);
            process::exit(1);
        }

        if self.is_register(data) {
            let reg_data = self.registers.get(reg).unwrap();
            let data_data = self.registers.get(data).unwrap();
            match reg_data {
                Data::Int(i) => {
                    match data_data {
                        Data::Int(j) => {
                            self.equal_flag = i == j;
                            self.greater_flag = i > j;
                            self.lesser_flag = i < j;
                        }

                        Data::Float(j) => {
                            self.equal_flag = *i as f64 == *j;
                            self.greater_flag = *i as f64 > *j;
                            self.lesser_flag = (*i as f64) < *j;
                        }

                        _ => {self.equal_flag = false; self.greater_flag = false; self.lesser_flag = false;}
                    }
                }

                Data::Bool(i) => {
                    match data_data {
                        Data::Bool(j) => {
                            self.equal_flag = i == j;
                            self.greater_flag = false;
                            self.lesser_flag = false;
                        }

                        _ => {self.equal_flag = false; self.greater_flag = false; self.lesser_flag = false;}
                    }
                }

                Data::Float(i) => {
                    match data_data {
                        Data::Int(j) => {
                            self.equal_flag = *i == *j as f64;
                            self.greater_flag = *i > *j as f64;
                            self.lesser_flag = *i < *j as f64;
                        }

                        Data::Float(j) => {
                            self.equal_flag = i == j;
                            self.greater_flag = i > j;
                            self.lesser_flag = i < j;
                        }

                        _ => {self.equal_flag = false; self.greater_flag = false; self.lesser_flag = false;}
                    }
                }

                Data::String(i) => {
                    match data_data {
                        Data::String(j) => {
                            self.equal_flag = i == j;
                            self.greater_flag = false;
                            self.lesser_flag = false;
                        }

                        _ => {self.equal_flag = false; self.greater_flag = false; self.lesser_flag = false;}
                    }
                }

                _ => {self.equal_flag = false; self.greater_flag = false; self.lesser_flag = false;}
            }
        }
        else {
            let reg_data = self.registers.get(reg).unwrap();
            let data_data = self.determine_type(data);
            match reg_data {
                Data::Int(i) => {
                    match data_data {
                        Data::Int(j) => {
                            self.equal_flag = *i == j;
                            self.greater_flag = *i > j;
                            self.lesser_flag = *i < j;
                        }

                        Data::Float(j) => {
                            self.equal_flag = *i as f64 == j;
                            self.greater_flag = *i as f64 > j;
                            self.lesser_flag = (*i as f64) < j;
                        }

                        _ => {self.equal_flag = false; self.greater_flag = false; self.lesser_flag = false;}
                    }
                }

                Data::Bool(i) => {
                    match data_data {
                        Data::Bool(j) => {
                            self.equal_flag = *i == j;
                            self.greater_flag = false;
                            self.lesser_flag = false;
                        }

                        _ => {self.equal_flag = false; self.greater_flag = false; self.lesser_flag = false;}
                    }
                }

                Data::Float(i) => {
                    match data_data {
                        Data::Int(j) => {
                            self.equal_flag = *i == j as f64;
                            self.greater_flag = *i > j as f64;
                            self.lesser_flag = *i < j as f64;
                        }

                        Data::Float(j) => {
                            self.equal_flag = *i == j;
                            self.greater_flag = *i > j;
                            self.lesser_flag = *i < j;
                        }

                        _ => {self.equal_flag = false; self.greater_flag = false; self.lesser_flag = false;}
                    }
                }

                Data::String(i) => {
                    match data_data {
                        Data::String(j) => {
                            self.equal_flag = *i == j;
                            self.greater_flag = false;
                            self.lesser_flag = false;
                        }

                        _ => {self.equal_flag = false; self.greater_flag = false; self.lesser_flag = false;}
                    }
                }

                _ => {self.equal_flag = false; self.greater_flag = false; self.lesser_flag = false;}
            }
        }
    }

    fn jmp(&mut self, label: &String) {
        if label == "debug" {self.debug(); return;}
        else if label == "print" {self.print(); return;}
        else if label == "printline" {self.printline(); return;}
        else if label == "input" {self.input(); return;}
        else if label == "exit" {self.exit(); return;}

        if !self.functions.contains_key(label) {
            eprintln!("[jmp] Attempted to jump to non-existant function: {}", label);
            process::exit(1);
        }

        let function = self.functions.get(label).unwrap().clone();
        for line in function {
            self.execute_line(&line);
        }
    }

    fn je(&mut self, label: &String) {
        if self.equal_flag {
            self.jmp(label);
        }
    }

    fn jne(&mut self, label: &String) {
        if !self.equal_flag {
            self.jmp(label);
        }
    }

    fn jg(&mut self, label: &String) {
        if self.greater_flag {
            self.jmp(label);
        }
    }

    fn jge(&mut self, label: &String) {
        if self.greater_flag || self.equal_flag {
            self.jmp(label);
        }
    }

    fn jl(&mut self, label: &String) {
        if self.greater_flag || self.equal_flag {
            self.jmp(label);
        }
    }

    fn jle(&mut self, label: &String) {
        if self.greater_flag || self.equal_flag {
            self.jmp(label);
        }
    }

    fn debug(&self) {
        println!("{:#?}", self);
    }

    fn print(&self) {
        print!("{}", self.registers.get("p0").unwrap().to_string());
        flush();
    }

    fn printline(&self) {
        println!("{}", self.registers.get("p0").unwrap().to_string());
    }

    fn input(&mut self) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        self.registers.insert("ret0".to_string(), Data::String(input.trim().to_string()));
    }

    fn exit(&self) {
        println!("Process exited with code '{}'", self.registers.get("p0").unwrap().to_string());
        process::exit(0);
    }

    fn run(&mut self) {
        if !self.functions.contains_key("main") {
            eprintln!("[FATAL] No main function found.");
            process::exit(1);
        }
        self.jmp(&"main".to_string());
    }
}

fn main() {
    loop {
        let mut input = String::new();
        print!("1. Run a file\n2. REPL mode\n3. Exit\n> ");
        flush();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => {
                input.clear();
                print!("Enter file name: ");
                flush();
                io::stdin().read_line(&mut input).unwrap();
                let file = Path::new(input.trim());
                if !file.exists() {
                    eprintln!("\nFile does not exist: {}\n", file.display());
                    continue;
                }
                let mut runtime = RunTime::new(read_to_string(file).unwrap());
                runtime.run();
            }
            "2" => {
                let mut repl_input = String::new();
                let mut runtime = RunTime::new(String::new());

                while repl_input.trim() != "exit" {
                    repl_input.clear();
                    print!("REPL> ");
                    flush();
                    io::stdin().read_line(&mut repl_input).unwrap();
                    if repl_input.trim() == "clear" {
                        print!("\x1B[2J\x1B[1;1H");
                        flush();
                    }
                    else if repl_input.trim() == "reset" {
                        runtime = RunTime::new(String::new());
                    }
                    else if repl_input.trim().is_empty() {
                        continue;
                    }
                    else {
                        runtime.execute_line(&repl_input.trim());
                    }
                }
            }
            "3" => {break;}
            _ => {println!("Invalid option.");}
        }
    }
}