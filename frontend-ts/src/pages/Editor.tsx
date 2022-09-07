import Button from "components/Button";
import { ReactElement, useState } from "react";
import { createEditor } from "slate"
import {Editable, withReact} from "slate-react"
import { BaseEditor, Descendant } from 'slate'
import { ReactEditor, Slate } from 'slate-react'

type CustomElement = { type: 'paragraph'; children: CustomText[] }
type CustomText = { text: string }

declare module 'slate' {
  interface CustomTypes {
    Editor: BaseEditor & ReactEditor
    Element: CustomElement
    Text: CustomText
  }
}

export default function Editor({file, closeFile}: {file: string, closeFile: () => void}): ReactElement {
    const [editor] = useState(() => withReact(createEditor()))
    return (<div className="p-2">
        <div>{file} <Button label="X" click={closeFile}/></div>
        <Slate editor={editor} value={[]}>
            <Editable></Editable>
            </Slate>
    </div>)
}