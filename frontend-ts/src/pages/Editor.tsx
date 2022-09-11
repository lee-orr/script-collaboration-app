import Button from 'components/Button'
import type { ReactElement } from 'react'
import { useCallback, useEffect, useMemo, useState } from 'react'
import { createEditor, Node, NodeEntry, Text, Location } from 'slate'
import type { RenderElementProps, RenderLeafProps } from 'slate-react'
import { Editable, withReact, Slate } from 'slate-react'
import { withYjs, YjsEditor } from '@slate-yjs/core'
import type { SyncedFile } from 'utils/SyncedFile'
import Input from 'components/Input'
import type { FileListing } from 'utils/FileList'
import withFountain from 'utils/SlateFountain'
import { Fountain } from 'fountain-js'


const serialize = (nodes: Node[]): string =>
	nodes.map(n => Node.string(n)).join('\n')

function previewFountain(content: string): string {
	const fountain = new Fountain()
	const output = fountain.parse(content)
	return `${output.html.title_page}<br/>${output.html.script}`
}

const ONE = 1

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

	useMemo((): void => {
		editor.onChange = (): void => {
			setVersion(v => v + ONE)
			setFountain(previewFountain(serialize(editor.children)))
		}
	}, [editor])

	const [value] = useState([])

	useEffect(() => {
		YjsEditor.connect(editor)
		return () => YjsEditor.disconnect(editor)
	}, [editor])

	const renderElement = useCallback(
		/* eslint-disable react/jsx-props-no-spreading */
		({ attributes, children, element }: RenderElementProps): JSX.Element => {
			let className = `slate-element slate-${element.type} `;
			let currentText = element.children.map((t): string => t.text).join('');
			if (currentText.startsWith('>') && currentText.endsWith('<')) {
				className += 'text-center'
			}
			switch (element.type) {
				case 'title_element':
					className += 'text-sm text-gray-400'
					break;
				case 'character':
					className += 'text-center text-lg'
					break;
				case 'dialogue':
					className += 'text-center'
					break;
				case 'lyrics':
					className += 'text-left pl-[20%]'
					break;
				case 'scene_header':
					className += 'text-left text-2xl'
					break;
				case 'transition':
					className += 'text-right text-lg'
					break;
				case 'page_break':
					className += 'text-xs text-gray-400 border-b-2 border-gray-900'
					break;
				case 'boneyard':
					className += `text-xs bg-slate-900 pl-2 text-gray-400`
					break;
				default:
					break;
			}
			return (
			<p className={className} {...attributes}>{children}</p>
		)
			},
		[]
	)

	const renderLeaf = useCallback(({ attributes, children, leaf }: RenderLeafProps) => {
		if (leaf.text === '&nbsp;') {
			return <span {...attributes} className='text-xs text-gray-700'>{children}</span>
		}
		if (leaf.text.startsWith('(') && leaf.text.endsWith(')')) {
			return <span {...attributes} className='text-sm text-gray-400'>{children}</span>
		}

		let className = '';

		if (leaf.bold) className += ' font-bold'
		if (leaf.underline) className += ' underline'
		if (leaf.italic) className += ' italic'
		if (leaf.note) className += ' bg-slate-900 text-gray-400'
		
		return (
		  <span
			{...attributes}
			className={className}
		  >
			{children}
		  </span>
		)
	  }, [])

	const decorate = useCallback(([node, path]: NodeEntry<Node>) => {
		if (!Text.isText(node))
			return []
		
		const ranges: Array<{bold?: boolean, underline?: boolean, italic?: boolean, note?: boolean, anchor: Location, focus: Location}> = [];

		const text = node.text;

		let start = 0;
		let end = 0;
		let currentState = {
			bold: false,
			underline: false,
			italic: false,
			note: false
		}

		function setRange(update: {bold?: boolean, underline?: boolean, italic?: boolean, note?: boolean}) {
			ranges.push({
				...currentState,
				anchor: {path, offset: start},
				focus: {path, offset: end}
			})
			start = end;
			currentState = {...currentState, ...update}
		}

		for (let i = 0; i < text.length; i++) {
			if (text[i] === '_') {
				if (currentState.underline) {
					end = i + 1
				}
				setRange({ underline: !currentState.underline})
			}
			if (text[i] === '*' && text[i + 1] === '*' && text[i + 2] === '*') {
				end = i + 3
				setRange({ bold: !currentState.bold, italic: !currentState.italic})
				i = i + 3
			}
			if (text[i] === '*' && text[i + 1] === '*') {
				end = i + 1
				setRange({ bold: !currentState.bold})
				i = i + 1
			}
			if (text[i] === '*') {
				if (currentState.italic) {
					end = i + 1
				}
				setRange({ italic: !currentState.italic})
			}
			if (text[i] === '[' && text[i + 1] === '[') {
				end = i
				setRange({ note: true})
				i = i + 1
			}
			if (text[i] === ']' && text[i - 1] === ']') {
				end = i + 1
				setRange({note: false})
			}
			end = i
		}
		
		setRange({})

		return ranges
	}, [])

	return (
		<div className='flex flex-col items-stretch justify-start'>
			<div className='flex flex-row items-center justify-between border-b-2 border-b-slate-900 bg-slate-800 p-2'>
				<Input value={listing.name} input={renameFile} />{' '}
				<span className='flex flex-row justify-end gap-2'>
					<Button label='Preview' click={(): void => setPreview(!preview)} />
					<Button label='X' click={closeFile} />
				</span>
			</div>
			<div className='h-0 flex-grow p-2'>
				<div
					className='h-full w-full overflow-y-scroll'
					style={preview ? { position: 'fixed', bottom: '-200vh' } : {}}
				>
					<Slate editor={editor} value={value}>
						<Editable renderElement={renderElement} renderLeaf={renderLeaf} decorate={decorate} />
					</Slate>
				</div>
				{preview ? (
					<div
						key={version}
						className='script h-full w-full overflow-y-scroll'
						/* eslint-disable react/no-danger */
						dangerouslySetInnerHTML={{ __html: fountain }}
					/>
				) : (
					''
				)}
			</div>
		</div>
	)
}
