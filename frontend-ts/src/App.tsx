import LoadingOrError from 'components/LoadingOrError'
import type { ReactElement } from 'react'
import { lazy, Suspense } from 'react'
import { BrowserRouter, Route, Routes } from 'react-router-dom'

const MenuPage = lazy(async () => import('pages/Menu'))
const Join = lazy(async () => import('pages/Join'))
const Host = lazy(async () => import('pages/Host'))

export default function App(): ReactElement {
	return (
		<BrowserRouter>
			<Suspense fallback={<LoadingOrError />}>
				<Routes>
					<Route path='/' element={<MenuPage />} />
					<Route path='/join' element={<Join />} />
					<Route path='/host' element={<Host />} />
				</Routes>
			</Suspense>
		</BrowserRouter>
	)
}
