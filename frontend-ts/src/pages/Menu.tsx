import Head from 'components/Head'
import LoadingOrError from 'components/LoadingOrError'
import type { ReactElement } from 'react'
import { useQuery } from '@tanstack/react-query'
import Button from "components/Button"

export default function MenuPage(): ReactElement {

	return (
		<>
			<Head title='Script Editor' />
			<div className='flex flex-col justify-center h-screen items-center gap-4'>
				<h1 className='text-4xl font-bold'>Script Editor</h1>
				<Button onClick={() => {}} label="Host Session"/>
				<Button onClick={() => {}} label="Join Session"/>
			</div>
		</>
	)
}
