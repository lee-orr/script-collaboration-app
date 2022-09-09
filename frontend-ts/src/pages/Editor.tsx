import Button from 'components/Button'
import { ReactElement, useCallback } from 'react'
import { useEffect, useMemo, useState } from 'react'
import type { BaseEditor } from 'slate'
import { createEditor, Node } from 'slate'
import type { ReactEditor, RenderElementProps } from 'slate-react'
import { Editable, withReact, Slate } from 'slate-react'
import { withYjs, YjsEditor } from '@slate-yjs/core'
import type { SyncedFile } from 'utils/SyncedFile'
import Input from 'components/Input'
import type { FileListing } from 'utils/FileList'
import withFountain from 'utils/SlateFountain'
import { Fountain } from "fountain-js"


const serialize = (nodes:Node[]) : string => {
  return nodes.map(n => Node.string(n)).join('\n')
}

function previewFountain(content: string): string {
	const fountain = new Fountain()
	const output = fountain.parse(content)
	return output.html.title_page + '<br/>' + output.html.script
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
	const [preview, setPreview] = useState(false)
	const { content } = useMemo(() => file.connect(), [file])
	const [version, setVersion] = useState(0)
	const [fountain, setFountain] = useState('')


	const editor = useMemo(
		() => withReact(withYjs(withFountain(createEditor()), content)),
		[content]
	)


	useMemo(() => {
		editor.onChange =  () => {
			setVersion(version + 1)
			setFountain(previewFountain(serialize(editor.children)))
		};
	}, [file])

	const [value] = useState([])

	useEffect(() => {
		YjsEditor.connect(editor)
		return () => YjsEditor.disconnect(editor)
	}, [editor])

  const renderElement = useCallback(({ attributes, children, element }: RenderElementProps): JSX.Element => {
    return (<p {...attributes}>{children}</p>)
  }, [])


	return (
		<div className='flex flex-col items-stretch justify-start'>
			<div className='flex flex-row items-center justify-between border-b-2 border-b-slate-900 bg-slate-800 p-2'>
				<Input value={listing.name} input={renameFile} />{' '}
				<span className='flex flex-row justify-end gap-2'>
					<Button label='Preview' click={() => setPreview(!preview)}/>
					<Button label='X' click={closeFile} />
				</span>
			</div>
			<div className='flex-grow p-2 h-0'>
				<div className='w-full h-full overflow-y-scroll' style={preview ? { position: 'fixed', bottom: '-200vh' } : {}}>
					<Slate editor={editor} value={value}>
						<Editable/>
					</Slate>
				</div>
				{preview ? <div key={version} className='w-full h-full overflow-y-scroll script' dangerouslySetInnerHTML={{__html: fountain}}></div> : <></>}
			</div>
		</div>
	)
}
