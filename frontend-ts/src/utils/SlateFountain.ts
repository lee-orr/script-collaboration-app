import type { Editor, BaseEditor } from 'slate'
import type { ReactEditor } from 'slate-react'

interface TitleElement {
	type: 'title'
	children: CustomText[]
}

interface TitleElementKey {
	type: 'title_element_key'
	children: CustomText[]
}

interface TitleElementContent {
	type: 'title_element_content'
	children: CustomText[]
}

interface RawText {
	type: 'raw'
	children: CustomText[]
}

interface CustomText {
	text: string
}

declare module 'slate' {
	interface CustomTypes {
		Editor: BaseEditor & ReactEditor
		Element: RawText | TitleElement | TitleElementContent | TitleElementKey
		Text: CustomText
	}
}

const withFountain = (editor: Editor): Editor => {
	const { normalizeNode } = editor

	/* eslint-disable no-param-reassign */
	editor.normalizeNode = (entry): void => {
		normalizeNode(entry)
	}

	return editor
}

export default withFountain
