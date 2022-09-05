import Head from 'components/Head'
import type { ReactElement } from 'react'
import Button from 'components/Button'
import { useNavigate } from 'react-router-dom'

export default function MenuPage(): ReactElement {
	const nav = useNavigate()
	return (
		<>
			<Head title='Script Editor' />
			<div className='flex h-screen flex-col items-center justify-center gap-4'>
				<h1 className='text-4xl font-bold'>Script Editor</h1>
				<Button
					click={() : void => {
						nav('/host')
					}}
					label='Host Session'
				/>
				<Button
					click={() : void => {
						nav('/join')
					}}
					label='Join Session'
				/>
			</div>
		</>
	)
}
