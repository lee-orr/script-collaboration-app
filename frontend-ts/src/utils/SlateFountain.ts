import type { Editor, BaseEditor } from 'slate'
import { Path, Node, Element, Transforms } from 'slate'
import type { ReactEditor } from 'slate-react'

interface TitleElement {
	type: 'title_element'
	children: CustomText[]
}

interface SceneHeader {
	type: 'scene_header'
	children: CustomText[]
}

interface Transition {
	type: 'transition'
	children: CustomText[]
}

interface CharacterElement {
	type: 'character'
	children: CustomText[]
}

interface DialogueElement {
	type: 'dialogue'
	children: CustomText[]
}

interface LyricElement {
	type: 'lyrics'
	children: CustomText[]
}

interface RawText {
	type: 'raw'
	children: CustomText[]
}

interface Empty {
	type: 'empty'
	children: []
}

interface PageBreak {
	type: 'page_break'
	children: CustomText[]
}

interface BoneYard {
	type: 'boneyard'
	containsStart: boolean
	containsEnd: boolean
	children: CustomText[]
}

interface CustomText {
	text: string
	bold?: boolean
	underline?: boolean
	italic?: boolean
	note?: boolean
}

declare module 'slate' {
	interface CustomTypes {
		Editor: BaseEditor & ReactEditor
		Element: { edit?: boolean } & (
			| BoneYard
			| CharacterElement
			| DialogueElement
			| Empty
			| LyricElement
			| PageBreak
			| RawText
			| SceneHeader
			| TitleElement
			| Transition
		)
		Text: CustomText
	}
}

function ProcessTitle(
	node: Element,
	editor: Editor,
	path: Path,
	currentText: string,
	previous_element: Element | false
): boolean {
	if (
		// eslint-disable-next-line @typescript-eslint/no-magic-numbers
		path.length === 1 &&
		path[0] === 0 &&
		node.type !== 'title_element' &&
		currentText.includes(':')
	) {
		Transforms.setNodes(
			editor,
			{ type: 'title_element', children: [{ text: currentText }] },
			{ at: path }
		)
		return true
	}
	if (
		previous_element &&
		previous_element.type === 'title_element' &&
		currentText.length > 0 &&
		node.type !== 'title_element'
	) {
		Transforms.setNodes(
			editor,
			{ type: 'title_element', children: [{ text: currentText }] },
			{ at: path }
		)
		return true
	}
	if (
		node.type === 'title_element' &&
		previous_element &&
		previous_element.type !== 'title_element'
	) {
		Transforms.setNodes(
			editor,
			{ type: 'raw', children: [{ text: currentText }] },
			{ at: path }
		)
		return true
	}
	return false
}

function ProcessDialogue(
	node: Element,
	editor: Editor,
	path: Path,
	currentText: string,
	previous_element: Element | false
): boolean {
	if (
		currentText.length === 0 ||
		node.type === 'transition' ||
		node.type === 'scene_header'
	)
		return false
	if (
		(!previous_element || previous_element.type === 'empty') &&
		(currentText.startsWith('@') ||
			currentText === currentText.toUpperCase()) &&
		node.type !== 'character'
	) {
		Transforms.setNodes(
			editor,
			{ type: 'character', children: [{ text: currentText }] },
			{ at: path }
		)
		return true
	}
	if (
		(!previous_element || previous_element.type === 'empty') &&
		currentText !== currentText.toUpperCase() &&
		!currentText.startsWith('@') &&
		node.type === 'character'
	) {
		Transforms.setNodes(
			editor,
			{ type: 'raw', children: [{ text: currentText }] },
			{ at: path }
		)
		return true
	}

	if (
		currentText.startsWith('~') &&
		previous_element &&
		(previous_element.type === 'character' ||
			previous_element.type === 'dialogue' ||
			previous_element.type === 'lyrics') &&
		node.type !== 'lyrics'
	) {
		Transforms.setNodes(
			editor,
			{ type: 'lyrics', children: [{ text: currentText }] },
			{ at: path }
		)
		return true
	}
	if (
		node.type === 'lyrics' &&
		(!previous_element ||
			(previous_element.type !== 'character' &&
				previous_element.type !== 'dialogue' &&
				previous_element.type !== 'lyrics') ||
			!currentText.startsWith('~'))
	) {
		Transforms.setNodes(
			editor,
			{ type: 'raw', children: [{ text: currentText }] },
			{ at: path }
		)
		return true
	}

	if (
		previous_element &&
		(previous_element.type === 'character' ||
			previous_element.type === 'dialogue' ||
			previous_element.type === 'lyrics') &&
		node.type !== 'dialogue' &&
		node.type !== 'lyrics'
	) {
		Transforms.setNodes(
			editor,
			{ type: 'dialogue', children: [{ text: currentText }] },
			{ at: path }
		)
		return true
	}
	if (
		node.type === 'dialogue' &&
		(!previous_element ||
			(previous_element.type !== 'character' &&
				previous_element.type !== 'dialogue' &&
				previous_element.type !== 'lyrics'))
	) {
		Transforms.setNodes(
			editor,
			{ type: 'raw', children: [{ text: currentText }] },
			{ at: path }
		)
		return true
	}
	return false
}

const sceneHeaders = ['.', 'INT', 'EXT', 'EST', 'INT./EXT', 'INT/EXT', 'I/E']

function ProcessHeadingsAndTransitions(
	node: Element,
	editor: Editor,
	path: Path,
	currentText: string
): boolean {
	if (currentText.length === 0) return false
	const uppercase = currentText.toUpperCase()
	if (
		node.type !== 'scene_header' &&
		sceneHeaders.map(h => uppercase.startsWith(h)).includes(true)
	) {
		Transforms.setNodes(
			editor,
			{ type: 'scene_header', children: [{ text: currentText }] },
			{ at: path }
		)
		return true
	}
	if (
		node.type === 'scene_header' &&
		!sceneHeaders.map(h => uppercase.startsWith(h)).includes(true)
	) {
		Transforms.setNodes(
			editor,
			{ type: 'raw', children: [{ text: currentText }] },
			{ at: path }
		)
		return true
	}

	if (
		node.type !== 'transition' &&
		((currentText.startsWith('>') && !currentText.endsWith('<')) ||
			(currentText.endsWith('TO:') && currentText === uppercase))
	) {
		Transforms.setNodes(
			editor,
			{ type: 'transition', children: [{ text: currentText }] },
			{ at: path }
		)
		return true
	}
	if (
		node.type === 'transition' &&
		(!currentText.startsWith('>') || currentText.endsWith('<')) &&
		(!currentText.endsWith('TO:') || currentText !== uppercase)
	) {
		Transforms.setNodes(
			editor,
			{ type: 'raw', children: [{ text: currentText }] },
			{ at: path }
		)
		return true
	}
	return false
}

function ProcessArbitraryControls(
	node: Element,
	editor: Editor,
	path: Path,
	currentText: string,
	previous_element: Element | false
): boolean {
	const pageBreak = /^===+$/.test(currentText)
	if (pageBreak && node.type !== 'page_break') {
		Transforms.setNodes(
			editor,
			{ type: 'page_break', children: [{ text: currentText }] },
			{ at: path }
		)
		return true
	}
	if (!pageBreak && node.type === 'page_break') {
		Transforms.setNodes(
			editor,
			{ type: 'raw', children: [{ text: currentText }] },
			{ at: path }
		)
		return true
	}
	if (
		currentText.startsWith('/*') &&
		currentText.endsWith('*/') &&
		node.type !== 'boneyard'
	) {
		Transforms.setNodes(
			editor,
			{ type: 'boneyard', containsStart: true, containsEnd: true },
			{ at: path }
		)
		return true
	}
	if (currentText.startsWith('/*') && node.type !== 'boneyard') {
		Transforms.setNodes(
			editor,
			{ type: 'boneyard', containsStart: true, containsEnd: false },
			{ at: path }
		)
		return true
	}
	if (
		currentText.endsWith('*/') &&
		(node.type !== 'boneyard' || !node.containsEnd) &&
		previous_element &&
		previous_element.type === 'boneyard'
	) {
		Transforms.setNodes(
			editor,
			{ type: 'boneyard', containsEnd: true, containsStart: false },
			{ at: path }
		)
		return true
	}
	if (
		previous_element &&
		previous_element.type === 'boneyard' &&
		!previous_element.containsEnd &&
		node.type !== 'boneyard'
	) {
		Transforms.setNodes(
			editor,
			{ type: 'boneyard', containsStart: false, containsEnd: false },
			{ at: path }
		)
		return true
	}
	if (
		node.type === 'boneyard' &&
		node.containsStart &&
		!currentText.startsWith('/*')
	) {
		Transforms.setNodes(
			editor,
			{ type: 'raw', children: [{ text: currentText }] },
			{ at: path }
		)
		return true
	}
	if (
		node.type === 'boneyard' &&
		!node.containsStart &&
		(!previous_element ||
			previous_element.type !== 'boneyard' ||
			previous_element.containsEnd)
	) {
		Transforms.setNodes(
			editor,
			{ type: 'raw', children: [{ text: currentText }] },
			{ at: path }
		)
		return true
	}

	return false
}

const withFountain = (editor: Editor): Editor => {
	const { normalizeNode } = editor

	/* eslint-disable no-param-reassign */
	editor.normalizeNode = (entry): void => {
		const [node, path] = entry

		if (Element.isElement(node)) {
			const previousPath = Path.hasPrevious(path) && Path.previous(path)
			const previousElement = previousPath
				? (Node.get(editor, previousPath) as Element)
				: false
			const nextPath = Path.next(path)
			let nextElement: Element | false = false
			try {
				nextElement = Node.get(editor, nextPath) as Element
				/* eslint-disable-next-line no-empty */
			} catch {}
			const currentText = node.children.map((t): string => t.text).join('')
			let edited = false

			if (
				node.type !== 'empty' &&
				currentText.length === 0 &&
				(!previousElement || previousElement.type !== 'boneyard')
			) {
				Transforms.setNodes(
					editor,
					{ type: 'empty', children: [] },
					{ at: path }
				)
				edited = true
			}
			if (!edited)
				edited = ProcessArbitraryControls(
					node,
					editor,
					path,
					currentText,
					previousElement
				)

			if (currentText.length > 0 && node.type !== 'boneyard') {
				if (node.type === 'empty') {
					Transforms.setNodes(editor, { type: 'raw' }, { at: path })
					edited = true
				}
				if (!edited)
					edited = ProcessTitle(
						node,
						editor,
						path,
						currentText,
						previousElement
					)

				if (!edited)
					edited = ProcessHeadingsAndTransitions(
						node,
						editor,
						path,
						currentText
					)

				if (!edited)
					edited = ProcessDialogue(
						node,
						editor,
						path,
						currentText,
						previousElement
					)
			}
			if (edited) {
				if (nextElement)
					Transforms.setNodes(
						editor,
						{ edit: !nextElement.edit },
						{ at: nextPath }
					)
				return
			}
		}

		normalizeNode(entry)
	}

	return editor
}

export default withFountain
