import React, { useEffect, useState } from "react";
import cx from "clsx";
import { useApp } from "../context/app-state";
import type { Widget } from "../types";
import { useWidgets } from "../context/widgets";
import style from "./Details.module.scss";
import { useSettings } from "../context/settings";
import DetailsInput from "../widgets/DetailsInput";
import { useApi } from "../context/wasm-api";

export default function Details(): JSX.Element | null {
    const [{ update_settings, update_widget }] = useApi();
    const [app] = useApp();
    const [widgets] = useWidgets();
    const [settings] = useSettings();
    const [keepRatio, setKeepRatio] = useState<boolean>();
    const [currentWidget, setCurrentWidget] = useState<Widget>();
    const [x, setX] = useState<number>();
    const [y, setY] = useState<number>();
    const [width, setWidth] = useState<number>();
    const [height, setHeight] = useState<number>();

    useEffect(() => {
        if (!app) return;
        setCurrentWidget(widgets.find((widget) => widget.uuid === app.active_widget_uuid));
        setKeepRatio(app.settings.keep_ratio || false);
    }, [widgets, app]);

    useEffect(() => {
        if (currentWidget) {
            setX(currentWidget.geometry.x);
            setY(currentWidget.geometry.y);
            setWidth(currentWidget.geometry.width);
            setHeight(currentWidget.geometry.height);
        }
    }, [currentWidget?.geometry]);

    function toggleKeepRatio() {
        if (update_settings && app) {
            update_settings({
                ...app.settings,
                keep_ratio: !app.settings.keep_ratio,
            });
        }
    }

    function onChange(prop: string) {
        return function (value: number) {
            if (currentWidget && update_widget) {
                update_widget({
                    ...currentWidget,
                    geometry: {
                        ...currentWidget.geometry,
                        [prop]: value,
                    },
                });
            }
        };
    }

    if (!currentWidget || settings.hideUserInterface) return null;

    return (
        <div className={style.panel}>
            <div className={style.part}>
                <div className={style.title}>Geometry</div>
                <div className={style.group}>
                    <DetailsInput label={"X"} unit={"px"} value={x} onChange={onChange("x")} />
                    <DetailsInput label={"Y"} unit={"px"} value={y} onChange={onChange("xy")} />
                    <DetailsInput
                        label={"°"}
                        unit={"px"}
                        value={0}
                        onChange={() => {}}
                        isDisabled
                    />
                </div>
                <div className={style.group}>
                    <DetailsInput
                        label={"W"}
                        unit={"px"}
                        value={width}
                        min={1}
                        isLockedTo={keepRatio ? "right" : undefined}
                        onChange={onChange("width")}
                    />
                    <div className={style.icon} onClick={() => toggleKeepRatio()}>
                        {keepRatio ? (
                            <i className={cx("fad", "fa-lock-alt", style.iconActive)} />
                        ) : (
                            <i className="fad fa-unlock-alt" />
                        )}
                    </div>
                    <DetailsInput
                        label={"H"}
                        unit={"px"}
                        value={height}
                        min={1}
                        isLockedTo={keepRatio ? "left" : undefined}
                        onChange={onChange("height")}
                    />
                </div>
            </div>
        </div>
    );
}
