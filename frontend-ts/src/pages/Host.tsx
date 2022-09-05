import Button from 'components/Button'
import Input from 'components/Input'
import type { ReactElement } from 'react'
import { useNavigate } from 'react-router-dom'

const BACK_INDEX = -1

export default function HostPage(): ReactElement {
	const nav = useNavigate()
	return (
		<div className='flex h-screen flex-col items-center justify-center gap-4'>
				<h1 className='text-4xl font-bold'>Host Session</h1>
				<span className=''>Your Display Name:</span>
				<Input value='' input={(): void => {}} />
				<Button click={(): void => {}} label='Choose The Hosted Folder' />
				<Button
					click={() : void=> {
						nav(BACK_INDEX)
					}}
					label='Back'
				/>
			</div>
	)
}
