import { Editor, BaseEditor, Path, Node, Element, Transforms } from 'slate'
import type { ReactEditor } from 'slate-react'
interface TitleElement {
	type: 'title_element'
	children: CustomText[]
}

interface RawText {
	type: 'raw'
	children: CustomText[]
}

interface Empty {
	type: 'empty',
	children: []
}

interface CustomText {
	text: string
}

declare module 'slate' {
	interface CustomTypes {
		Editor: BaseEditor & ReactEditor
		Element: {edit?: boolean} & (RawText | TitleElement | Empty)
		Text: CustomText
	}
}

const withFountain = (editor: Editor): Editor => {
	const { normalizeNode } = editor

	/* eslint-disable no-param-reassign */
	editor.normalizeNode = (entry): void => {
		const [node, path] = entry

		if (Element.isElement(node)) {
			let previous_path = (Path.hasPrevious(path) && Path.previous(path))
			let previous_element = previous_path ? Node.get(editor, previous_path) as Element : false
			let next_path = Path.next(path)
			let next_element: Element | false = false;
			try {
				next_element = Node.get(editor, next_path) as Element
			} catch {}
			let currentText = node.children.map((t): string => t.text).join('');
			let edited = false

			if (node.type !== "empty" && node.children.length === 1 && node.children[0].text === '') {
				Transforms.setNodes(editor, { type: "empty", children: [] }, { at: path })
				edited = true
			} 

			if (!edited)
				edited = ProcessTitle(node, editor, path, currentText, previous_element)

			if (edited) {
				if (next_element) Transforms.setNodes(editor, { edit: !next_element.edit}, { at: next_path})
				return
			}
		}

		normalizeNode(entry)
	}

	return editor
}

export default withFountain

function ProcessTitle(node: Element, editor: Editor, path: Path, currentText: string, previous_element: false | Element): boolean {
	if (path.length === 1 && path[0] === 0 && node.type !== 'title_element' && currentText.includes(':')) {
		Transforms.setNodes(editor, { type: 'title_element', children: [{ text: currentText }] }, { at: path })
		return true
	} else if (previous_element && previous_element.type === 'title_element' && currentText.length > 0 && node.type !== 'title_element') {
		Transforms.setNodes(editor, { type: 'title_element', children: [{ text: currentText }] }, { at: path })
		return true
	} else if (node.type === 'title_element' && previous_element && previous_element.type !== 'title_element') {
		Transforms.setNodes(editor, { type: 'raw', children: [{ text: currentText }] }, { at: path })
		return true
	}
	return false
}

