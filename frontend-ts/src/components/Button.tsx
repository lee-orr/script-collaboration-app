import type { ReactElement } from 'react'

export default function Button({
	click: onClick,
	label,
	disabled = false
}: {
	click: () => void
	label: string
	disabled?: boolean
}): ReactElement {
	return (
		<button
			type='button'
			className='bg-slate-900 p-2 text-gray-200 hover:bg-slate-700 active:bg-slate-800 disabled:bg-slate-700 disabled:text-slate-500'
			onClick={onClick}
			disabled={disabled}
		>
			{label}
		</button>
	)
}

Button.defaultProps = {
	disabled: false
}
