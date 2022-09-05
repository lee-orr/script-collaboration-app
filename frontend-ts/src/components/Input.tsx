import type { ReactElement } from 'react'

export default function Input({
	value,
	input: onInput
}: {
	value: string
	input: (value_: string) => void
}): ReactElement {
	return (
		<input
			className='border-b-2 border-b-slate-400 bg-transparent p-1'
			value={value}
			onInput={(event): void => onInput(event.currentTarget.value)}
		/>
	)
}
