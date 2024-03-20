export declare interface Gradient {
    from: string,
    to: string,
    orientation?: GradientOrientation
}

export declare type GradientOrientation = "bg-gradient-to-t" | "bg-gradient-to-r" | "bg-gradient-to-b" | "bg-gradient-to-l" | "bg-gradient-to-tl" | "bg-gradient-to-tr" | "bg-gradient-to-bl" | "bg-gradient-to-br";