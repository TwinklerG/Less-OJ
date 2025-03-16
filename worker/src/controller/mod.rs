use rocket::form::Form;

use crate::service::{sandbox_service, Config};

#[derive(FromForm)]
pub struct UploadForm {
    src: String,
    image: String,
    stdin: String,
}

#[post("/c", data = "<upload_form>")]
pub fn c_controller(upload_form: Form<UploadForm>) -> String {
    let sandbox_result = sandbox_service(
        &upload_form.src,
        "main.c",
        vec![
            r#"#!/bin/bash
gcc main.c -o main"#,
            r#"#!/bin/bash
./main"#,
        ],
        &Config::new(upload_form.image.clone(), upload_form.stdin.clone()),
    );
    toml::to_string(&sandbox_result).unwrap()
}

#[post("/cpp", data = "<upload_form>")]
pub fn cpp_controller(upload_form: Form<UploadForm>) -> String {
    let sandbox_result = sandbox_service(
        &upload_form.src,
        "main.cpp",
        vec![
            r#"#!/bin/bash
g++ main.cpp -o main"#,
            r#"#!/bin/bash
./main"#,
        ],
        &Config::new(upload_form.image.clone(), upload_form.stdin.clone()),
    );
    toml::to_string(&sandbox_result).unwrap()
}

#[get("/c")]
pub fn c_test_controller() -> String {
    let sandbox_result = sandbox_service(
        r#"#include <stdio.h>
int main() {
    int a, b;
    scanf("%d%d", &a, &b);
    printf("C: a + b = %d\n", a + b);
}"#,
        "main.c",
        vec![
            r#"#!/bin/bash
gcc main.c -o main"#,
            r#"#!/bin/bash
./main"#,
        ],
        &Config::new("gcc:14.2".into(), "1 2".to_string()),
    );
    toml::to_string(&sandbox_result).unwrap()
}

#[get("/cpp")]
pub fn cpp_test_controller() -> String {
    let sandbox_result = sandbox_service(
        r#"#include <iostream>
using namespace std;
int main() {
    int a, b;
    cin >> a >> b;
    cout << "C++: a + b = " << a + b << "\n";
}"#,
        "main.cpp",
        vec![
            r#"#!/bin/bash
g++ main.cpp -o main"#,
            r#"#!/bin/bash
./main"#,
        ],
        &Config::new("gcc:14.2".into(), "2 3".to_string()),
    );
    toml::to_string(&sandbox_result).unwrap()
}

#[get("/java")]
pub fn java_test_controller() -> String {
    let mut config = Config::new("openjdk:11".into(), "3 4".to_string());
    config.memory_limit = 4096000;
    config.memory_reserved = 4096000;
    let sandbox_result = sandbox_service(
        r#"import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Integer a, b;
        Scanner sc = new Scanner(System.in);
        a = sc.nextInt();
        b = sc.nextInt();
        Integer c = a + b;
        System.out.println("Java: a + b = " + c);
        sc.close();
    }
}"#,
        "Main.java",
        vec![
            r#"#!/bin/bash
javac -J-Xms64m -J-Xmx128m Main.java"#,
            r#"#!/bin/bash
java -Xms64m -Xmx128m Main"#,
        ],
        &config,
    );
    toml::to_string(&sandbox_result).unwrap()
}

#[get("/python3")]
pub fn python3_test_controller() -> String {
    let sandbox_result = sandbox_service(
        r#"a, b = list(int(x) for x in input().split())
print(f"a + b = {a + b}")"#,
        "main.py",
        vec![
            r#"#!/bin/bash
python main.py"#,
        ],
        &Config::new("python:3".into(), "4 5".to_string()),
    );
    toml::to_string(&sandbox_result).unwrap()
}

#[get("/go")]
pub fn go_test_controller() -> String {
    let mut config = Config::new("golang:1.24".into(), "5 6".to_string());
    config.time_limit = 5;
    config.memory_limit = 4096000;
    config
        .env
        .insert("GOCACHE".to_string(), "/tmp/.cache".to_string());
    let sandbox_result = sandbox_service(
        r#"package main

import "fmt"

func main() {
	var a, b int
	fmt.Scan(&a, &b)
	fmt.Println("a + b =", a + b)
}"#,
        "main.go",
        vec![
            r#"#!/bin/bash
go build main.go"#,
            r#"#!/bin/bash
./main"#,
        ],
        &config,
    );
    toml::to_string(&sandbox_result).unwrap()
}
