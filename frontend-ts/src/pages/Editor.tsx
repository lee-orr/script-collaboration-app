/* eslint @typescript-eslint/no-magic-numbers: 0 */
import Button from 'components/Button'
import type { ReactElement } from 'react'
import { useCallback, useEffect, useMemo, useState } from 'react'
import type { NodeEntry, BasePoint } from 'slate'
import { createEditor, Node, Text } from 'slate'
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
			let styles = `slate-element slate-${element.type} `
			const currentText = element.children.map((t): string => t.text).join('')
			if (currentText.startsWith('>') && currentText.endsWith('<')) {
				styles += 'text-center'
			}
			switch (element.type) {
				case 'title_element':
					styles += 'text-sm text-gray-400'
					break
				case 'lyrics':
					styles += 'text-left pl-[20%]'
					break
				case 'scene_header':
					break
				case 'transition':
					break
				case 'page_break':
					styles += 'text-xs text-gray-400 border-b-2 border-gray-900'
					break
				case 'boneyard':
					styles += `text-xs bg-slate-900 pl-2 text-gray-400`
					break
				default:
					break
			}
			return (
				<p className={styles} {...attributes}>
					{children}
				</p>
			)
		},
		[]
	)

	const renderLeaf = useCallback(
		({ attributes, children, leaf }: RenderLeafProps) => {
			if (
				leaf.text.trim() === '&nbsp' ||
				leaf.text.trim() === ';' ||
				leaf.text.trim() === '&nbsp;'
			) {
				return (
					<span {...attributes} className='text-gray-700'>
						{children}
					</span>
				)
			}

			let styles = ''

			if (leaf.bold) styles += ' font-bold'
			if (leaf.underline) styles += ' underline'
			if (leaf.italic) styles += ' italic'
			if (leaf.note) styles += ' bg-slate-900 text-gray-400'

			return (
				<span {...attributes} className={styles}>
					{children}
				</span>
			)
		},
		[]
	)

	const decorate = useCallback(([node, path]: NodeEntry) => {
		if (!Text.isText(node)) return []

		const ranges: {
			bold?: boolean
			underline?: boolean
			italic?: boolean
			note?: boolean
			anchor: BasePoint
			focus: BasePoint
		}[] = []

		const { text } = node

		let start = 0
		let end = 0
		let currentState = {
			bold: false,
			underline: false,
			italic: false,
			note: false
		}

		function setRange(update: {
			bold?: boolean
			underline?: boolean
			italic?: boolean
			note?: boolean
		}): void {
			ranges.push({
				...currentState,
				anchor: { path, offset: start },
				focus: { path, offset: end }
			})
			start = end
			currentState = { ...currentState, ...update }
		}

		for (let index = 0; index < text.length; index += 1) {
			if (text[index] === '_') {
				if (currentState.underline) {
					end = index + 1
				}
				setRange({ underline: !currentState.underline })
			}
			if (
				text[index] === '*' &&
				text[index + 1] === '*' &&
				text[index + 2] === '*'
			) {
				end = index + 3
				setRange({ bold: !currentState.bold, italic: !currentState.italic })
				index += 3
			}
			if (text[index] === '*' && text[index + 1] === '*') {
				end = index + 1
				setRange({ bold: !currentState.bold })
				index += 1
			}
			if (text[index] === '*') {
				if (currentState.italic) {
					end = index + 1
				}
				setRange({ italic: !currentState.italic })
			}
			if (text[index] === '[' && text[index + 1] === '[') {
				end = index
				setRange({ note: true })
				index += 1
			}
			if (text[index] === ']' && text[index - 1] === ']') {
				end = index + 1
				setRange({ note: false })
			}
			end = index
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
					className='h-full w-full overflow-y-scroll p-6'
					style={preview ? { position: 'fixed', bottom: '-200vh' } : {}}
				>
					<Slate editor={editor} value={value}>
						<Editable
							renderElement={renderElement}
							renderLeaf={renderLeaf}
							decorate={decorate}
						/>
					</Slate>
				</div>
				{preview ? (
					<div
						key={version}
						className='script h-full w-full overflow-y-scroll p-6'
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
