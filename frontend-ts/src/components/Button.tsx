import type { ReactElement } from 'react'

export default function Button({
	click: onClick,
	label
}: {
	click: () => void
	label: string
}): ReactElement {
	return (
		<button
			type="button"
			className='bg-slate-900 p-2 text-gray-200 hover:bg-slate-700 active:bg-slate-800'
			onClick={onClick}
		>
			{label}
		</button>
	)
}
