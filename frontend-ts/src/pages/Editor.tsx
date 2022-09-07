import Button from 'components/Button'
import type { ReactElement} from 'react';
import { useEffect, useMemo, useState } from 'react'
import { BaseEditor, Descendant, Element } from 'slate';
import { createEditor  } from 'slate'
import type { ReactEditor} from 'slate-react';
import { Editable, withReact , Slate } from 'slate-react'
import { withYjs, slateNodesToInsertDelta, YjsEditor } from '@slate-yjs/core'
import * as Y from 'yjs'
import { createInMemoryFile, SyncedFile } from 'utils/SyncedFile';
import Input from 'components/Input';

type RawText = {type: 'raw', children: CustomText[]}
type Paragraph = { type: 'paragraph'; children: CustomText[] };

type CustomElement = RawText | Paragraph
interface CustomText { text: string }

declare module 'slate' {
	interface CustomTypes {
		Editor: BaseEditor & ReactEditor
		Element: CustomElement
		Text: CustomText
	}
}

export default function Editor({
	file,
	closeFile
}: {
	file: SyncedFile
	closeFile: () => void
}): ReactElement {
  const [nameString, setNameString] = useState("")
  const [id, name, content] = useMemo(() => {
    const { id, name, content} = file.connect()
	setNameString(name.toString())
    return [id, name, content]
  }, [])

	const editor = useMemo(
		() => {
      return withReact(withYjs(createEditor(), content))
    },
		[]
	)

	const [value, onSetValue] = useState([])

	useEffect(() => {
		YjsEditor.connect(editor)
		return () => YjsEditor.disconnect(editor)
	}, [editor])

	useEffect(() => {
		const callback = () => {
			setNameString(name.toString())
		}
		name.observe(callback)
		return () => name.unobserve(callback)
	}, [name])

	return (
		<div className='flex flex-col justify-start items-stretch'>
			<div className="p-2 flex flex-row justify-between bg-slate-800 border-b-2 border-b-slate-900 items-center">
				<Input value={nameString} input={(value) => {
					name.delete(0, name.length)
					name.insert(0, value)
				}}  /> <Button label='X' click={closeFile} />
			</div>
			<Slate editor={editor} value={value}>
				<Editable />
			</Slate>
		</div>
	)
}
