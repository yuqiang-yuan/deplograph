import type { IStaticMethods } from "flyonui/flyonui";

declare global {
    interface Window {
        // FlyonUI
        HSStaticMethods: IStaticMethods;
    }

    namespace App {}
}

export {};
