import { ReactElement } from "react";

export default function Button({onClick, label}: {onClick: () => void, label: string}): ReactElement {
    return (<button className="bg-slate-900 text-gray-200 p-2 hover:bg-slate-700 active:bg-slate-800" onClick={onClick}>{label}</button>)
}