import { Transforms, Element, Node, Editor, BaseEditor, Path } from 'slate'
import { ReactEditor } from 'slate-react'

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
		Element: RawText | TitleElement | TitleElementKey | TitleElementContent
		Text: CustomText
	}
}

const withFountain = (editor: Editor): Editor => {
  const { normalizeNode } = editor

  editor.normalizeNode = (entry) : void => {
    const [node, path] = entry

    normalizeNode(entry)
  }

  return editor
}

export default withFountain