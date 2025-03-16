import { NextResponse } from "next/server"
import {parseTOML} from "confbox"

export const GET = async () => {
    const formData = new FormData();
    formData.append("src", `
#include <iostream>
using namespace std;
int main() {
int a, b;
cin >> a >> b;
cout << "a + b = " << a + b;
return 0;
}`)
    formData.append("image", "gcc:14.2")
    formData.append("stdin", "1 2")
    const resp = await fetch("http://127.0.0.1:8000/cpp", {
        method: "POST",
        body: formData
    })
    const data = parseTOML(await resp.text())
    return NextResponse.json(data);
}