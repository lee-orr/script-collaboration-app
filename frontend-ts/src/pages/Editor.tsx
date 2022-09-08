import Button from 'components/Button'
import type { ReactElement } from 'react'
import { useEffect, useMemo, useState } from 'react'
import type { BaseEditor } from 'slate'
import { createEditor } from 'slate'
import type { ReactEditor } from 'slate-react'
import { Editable, withReact, Slate } from 'slate-react'
import { withYjs, YjsEditor } from '@slate-yjs/core'
import type { SyncedFile } from 'utils/SyncedFile'
import Input from 'components/Input'
import type { FileListing } from 'utils/FileList'

interface RawText {
	type: 'raw'
	children: CustomText[]
}
interface Paragraph {
	type: 'paragraph'
	children: CustomText[]
}

interface CustomText {
	text: string
}

declare module 'slate' {
	interface CustomTypes {
		Editor: BaseEditor & ReactEditor
		Element: Paragraph | RawText
		Text: CustomText
	}
}

export default function Editor({
	listing,
	file,
	renameFile,
	closeFile
}: {
	listing: FileListing
	file: SyncedFile
	renameFile: (name: string) => void
	closeFile: () => void
}): ReactElement {
	const { content } = useMemo(() => file.connect(), [file])

	const editor = useMemo(
		() => withReact(withYjs(createEditor(), content)),
		[content]
	)

	const [value] = useState([])

	useEffect(() => {
		YjsEditor.connect(editor)
		return () => YjsEditor.disconnect(editor)
	}, [editor])

	return (
		<div className='flex flex-col items-stretch justify-start'>
			<div className='flex flex-row items-center justify-between border-b-2 border-b-slate-900 bg-slate-800 p-2'>
				<Input value={listing.name} input={renameFile} />{' '}
				<Button label='X' click={closeFile} />
			</div>
			<div className='flex-grow p-2'>
				<Slate editor={editor} value={value}>
					<Editable />
				</Slate>
			</div>
		</div>
	)
}
