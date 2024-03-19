export declare interface Gradient {
    from: string,
    to: string,
    orientation?: GradientOrientation
}

export declare type GradientOrientation = "t" | "r" | "b" | "l" | "tl" | "tr" | "bl" | "br";