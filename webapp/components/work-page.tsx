"use client";

import { CodeEditor } from "@/components/code-editor";
import { useState } from "react";
import { Separator } from "./ui/separator";
import { Button } from "./ui/button";

export function WorkPage() {
  const [code, setCode] = useState("");

  const data = {
    title: "两数之和",
    description: "给定两个整数a和b，求a+b",
    input: "两个整数，空格分开",
    output: "一个整数，表示两个整数的和",
    test_cases: [
      {
        input: "3 5",
        output: "8",
      },
      {
        input: "1 -2",
        output: "-1",
      },
    ],
  };

  return (
    <div className="flex lg:flex-row flex-col w-full h-full gap-4">
      <div className="flex flex-col gap-3 w-full md:w-1/2 h-full">
        <div className="text-3xl">{data.title}</div>
        <Separator className="mb-2" />
        <div className="text-2xl">题目描述</div>
        <div>{data.description}</div>
        <div className="text-2xl">输入格式</div>
        <div>{data.input}</div>
        <div className="text-2xl">输出格式</div>
        <div>{data.output}</div>
        <div className="text-2xl">测试样例</div>
        {data.test_cases.map((test_case, index) => (
          <div key={index} className="flex justify-around p-2 border-2">
            <span>{test_case.input}</span>
            <Separator orientation="vertical"></Separator>
            <span>{test_case.output}</span>
          </div>
        ))}
      </div>
      <div className="w-full p-2 lg:w-1/2 h-full flex flex-col">
        <CodeEditor language="c" code={code} setCode={setCode}></CodeEditor>
        <Button className="m-2">提交</Button>
      </div>
    </div>
  );
}
