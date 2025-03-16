"use client";

import { CodeEditor } from "@/components/code-editor";
import { useState } from "react";
import { Separator } from "./ui/separator";
import { Button } from "./ui/button";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "./ui/select";
import { Tabs, TabsList } from "./ui/tabs";
import { TabsContent, TabsTrigger } from "@radix-ui/react-tabs";

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
      <Tabs defaultValue="problem" className="w-full lg:w-1/2">
        <TabsList className="grid grid-cols-2 w-full">
          <TabsTrigger value="problem" className="hover:cursor-pointer">
            问题描述
          </TabsTrigger>
          <TabsTrigger value="result" className="hover:cursor-pointer">
            评测结果
          </TabsTrigger>
        </TabsList>
        <TabsContent value="problem">
          <div className="flex flex-col gap-3 w-full h-full">
            <div className="text-3xl">{data.title}</div>
            <Separator className="mb-2 w-full" />
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
        </TabsContent>
        <TabsContent value="result">测试结果</TabsContent>
      </Tabs>
      <div className="w-full p-2 lg:w-1/2 h-full flex flex-col">
        <CodeEditor language="c" code={code} setCode={setCode}></CodeEditor>
        <div className="w-full flex justify-center gap-4">
          <Select>
            <SelectTrigger className="w-[180px]">
              <SelectValue placeholder="Select a language" />
            </SelectTrigger>
            <SelectContent>
              <SelectGroup>
                <SelectLabel>Language</SelectLabel>
                <SelectItem value="c">C</SelectItem>
                <SelectItem value="cpp">C++</SelectItem>
                <SelectItem value="java">Java</SelectItem>
                <SelectItem value="python3">Python</SelectItem>
                <SelectItem value="go">Golang</SelectItem>
              </SelectGroup>
            </SelectContent>
          </Select>
          <Button
            className=""
            onClick={() => {
              console.log(code);
            }}
          >
            提交
          </Button>
        </div>
      </div>
    </div>
  );
}
