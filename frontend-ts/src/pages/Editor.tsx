import Button from 'components/Button'
import type { ReactElement} from 'react';
import { useEffect, useMemo, useState } from 'react'
import { BaseEditor, Descendant, Element } from 'slate';
import { createEditor  } from 'slate'
import type { ReactEditor} from 'slate-react';
import { Editable, withReact , Slate } from 'slate-react'
import { withYjs, slateNodesToInsertDelta, YjsEditor } from '@slate-yjs/core'
import * as Y from 'yjs'
import { createInMemoryFile } from 'utils/File';

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

const initialFile = createInMemoryFile("test", "some content")

export default function Editor({
	file,
	closeFile
}: {
	file: string
	closeFile: () => void
}): ReactElement {
  const [id, name, content] = useMemo(() => {
    const { id, name, content} = initialFile.connect()
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

	return (
		<div className='p-2'>
			<div>
				{file} <Button label='X' click={closeFile} />
			</div>
			<Slate editor={editor} value={value}>
				<Editable />
			</Slate>
		</div>
	)
}
