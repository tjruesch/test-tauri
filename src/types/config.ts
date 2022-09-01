export type subDir = {
    label: string
    displayName: string
    include: boolean
    description: string
}

export type config = {
    baseDir: string
    typeSelections: string[]
    presets: {
        web: Array<subDir>
        print: Array<subDir>
    }
    currentId: number
}