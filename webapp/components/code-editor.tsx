"use client";

import { Editor } from "@monaco-editor/react";
import { Dispatch, SetStateAction } from "react";

export function CodeEditor({
    language,
    code,
    setCode,
}: {
    language: string,
    code: string,
    setCode: Dispatch<SetStateAction<string>>
}) {
    return <div className="inline-block border-2 p-2 rounded-md border-gray-200">
        <Editor
            height="80vh"
            defaultLanguage={language} value={code} onChange={(val) => {
                setCode(val || "");
            }} />
    </div>

}