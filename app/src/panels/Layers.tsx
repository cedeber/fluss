import React, { useContext, useEffect, useRef, useState } from "react";
import style from "./Layers.module.scss";
import { useSettings } from "../context/settings";
import { useWidgets } from "../context/widgets";
import type { Widget } from "../types";
import cx from "clsx";
import { useApp } from "../context/app-state";
import Fuse from "fuse.js";
import { useApi } from "../context/wasm-api";
import { useFocus, useHover, useKeyboard, usePress } from "@react-aria/interactions";
import { FocusRing, useFocusRing } from "@react-aria/focus";
import type { AriaButtonProps } from "@react-types/button";
import { useButton } from "@react-aria/button";

type ReactState<T> = [T, React.Dispatch<React.SetStateAction<T>>];

// eslint-disable-next-line @typescript-eslint/no-empty-function
const SearchContext = React.createContext<ReactState<string>>(["", () => {}]);

export default function Layers(): JSX.Element {
    const searchState = useState<string>("");

    return (
        <SearchContext.Provider value={searchState}>
            <LayersPanel />
        </SearchContext.Provider>
    );
}

// --- Layers Panel ---
function LayersPanel(): JSX.Element | null {
    const [settings] = useSettings();
    let [widgets] = useWidgets();
    const [search] = useContext(SearchContext);

    if (search) {
        const fuse = new Fuse(widgets, { keys: ["name"] });
        widgets = widgets?.filter((widget) => {
            if (!fuse || search.trim() === "") return true;
            return fuse
                .search(search)
                .map((r) => r.item.uuid)
                .includes(widget.uuid);
        });
    }

    if (settings.hideUserInterface || !widgets || (!widgets.length && !search)) {
        return null;
    }

    return (
        <div className={style.panel}>
            <Tools />
            <div className={style.layers}>
                {widgets.map((widget) => (
                    <Layer key={widget.uuid} widget={widget} />
                ))}
                {search && widgets.length === 0 && (
                    <div className={style.notFound}>No layers with &quot;{search}&quot; found.</div>
                )}
            </div>
            <Search />
        </div>
    );
}

// --- Tools ---
export function Tools(): JSX.Element {
    const [api] = useApi();
    const [app] = useApp();
    const [widgets] = useWidgets();

    function onLayerSwap(uuid: string, a: number, b: number) {
        api.swap_widget({ uuid, a, b });
    }

    function onLayerMove(direction: number) {
        const len = widgets.length;
        let goTo = false;
        // FIXME: Avoid loop | Recode properly
        // Extract from and add to the beginning/end
        for (let i = 0; i < len; i += 1) {
            const widget = widgets[i];
            if (widget.uuid === app?.active_widget_uuid) {
                goTo = !goTo;
            }

            if (goTo) {
                if (Math.abs(direction) === 1) {
                    // up, down: single swap
                    onLayerSwap(widget.uuid, i, i + direction);
                    // don't swap next
                    // goTo = direction > 0;
                    goTo = false;
                } else if (direction > 0) {
                    // bottom: swap all next
                    onLayerSwap(widget.uuid, i, i + 1);
                    // don't swap next
                    goTo = false;
                } else {
                    // top
                    // swap all previous
                    for (let y = i; y > 0; y -= 1) {
                        onLayerSwap(widget.uuid, y, y - 1);
                    }
                    // don't swap next
                    goTo = false;
                }
            }
        }
    }

    return (
        <div className={style.layerTools}>
            <Tool iconClassName={"fa-bring-front"} onPress={() => onLayerMove(-2)} />
            <Tool iconClassName={"fa-bring-forward"} onPress={() => onLayerMove(-1)} />
            <Tool iconClassName={"fa-send-backward"} onPress={() => onLayerMove(1)} />
            <Tool iconClassName={"fa-send-back"} onPress={() => onLayerMove(2)} />
        </div>
    );
}

type ToolProps = {
    iconClassName: string;
};

function Tool(props: AriaButtonProps & ToolProps): JSX.Element {
    const ref = useRef<HTMLButtonElement>(null);
    const { buttonProps } = useButton(props, ref);
    const { focusProps, isFocusVisible } = useFocusRing();
    const { hoverProps, isHovered } = useHover({});

    return (
        <button
            className={cx(style.layerTool, style.icon, { focus: isFocusVisible, hover: isHovered })}
            ref={ref}
            {...buttonProps}
            {...focusProps}
            {...hoverProps}
        >
            <i className={cx("fad", "fa-md", props.iconClassName)} />
        </button>
    );
}

// --- Layer ---
type LayerProps = { widget: Widget };

function Layer({ widget }: LayerProps): JSX.Element | null {
    const [app] = useApp();
    const [api] = useApi();
    const ref = useRef<HTMLDivElement>(null);
    const inputRef = useRef<HTMLInputElement>(null);
    const [editMode, setEditMode] = useState<boolean>(false);
    const [name, setName] = useState<string>(widget.name);
    const { hoverProps } = useHover({
        onHoverChange: (isHovering) => {
            if (isHovering) api.hover_widget(widget.uuid);
            else api.hover_widget(null);
        },
    });
    const { focusProps } = useFocus({
        onFocus: () => api.select_widget(widget.uuid),
    });
    const { pressProps } = usePress({
        onPress: () => {
            api.toggle_visibility_widget(widget.uuid);
        },
    });

    useEffect(() => {
        if (widget.uuid === app?.active_widget_uuid) {
            ref.current?.focus();
        } else {
            setEditMode(false);
        }
    }, [app?.active_widget_uuid]);

    useEffect(() => {
        if (editMode) {
            inputRef.current?.select();
            api.activate_events(false);
        } else {
            // FIXME: Fucking fails twice and after it's ok...
            try {
                api.activate_events(true);
            } catch {
                /* empty */
            }
        }
    }, [editMode]);

    if (!app) {
        return null;
    }

    function updateName(event: React.KeyboardEvent<HTMLInputElement>) {
        if (event.key === "Escape") {
            setName(widget.name);
            setEditMode(false);
        } else if (event.key === "Enter") {
            api.update_widget({
                ...widget,
                name,
            });
            setEditMode(false);
        }
    }

    return (
        <div
            className={cx(style.layer, {
                [style.selected]: widget.uuid === app.active_widget_uuid,
                [style.hovered]: widget.uuid === app.pointer_widget_uuid,
                [style.hidden]: !widget.visible,
                [style.edit]: editMode,
            })}
            {...hoverProps}
            {...focusProps}
            tabIndex={0}
            role="button"
            ref={ref}
        >
            <div className={style.layerName}>
                <i className={cx(style.layerIcon, "fad", "fa-vector-square", "fa-sm")} />
                {editMode ? (
                    <div className={style.edit}>
                        <input
                            value={name}
                            ref={inputRef}
                            onChange={(event) => setName(event.currentTarget.value)}
                            onKeyDown={updateName}
                        />
                    </div>
                ) : (
                    <div className={style.name} onDoubleClick={() => setEditMode(true)}>
                        {widget.name || " "}
                    </div>
                )}
            </div>
            <div className={style.tool} {...pressProps}>
                <i className="fad fa-eye-slash fa-sm" />
            </div>
        </div>
    );
}

// --- Search ---
function Search() {
    const [search, setSearch] = useContext(SearchContext);
    const [api] = useApi();
    const ref = useRef<HTMLInputElement>(null);
    const { focusProps } = useFocus({
        onFocus: () => api.activate_events(false),
        onBlur: () => api.activate_events(true),
    });
    const { keyboardProps } = useKeyboard({
        onKeyDown: (e) => {
            if (e.key === "Escape") setSearch("");
        },
    });
    const { pressProps } = usePress({
        onPress: () => {
            setSearch("");
            ref.current?.focus();
        },
    });

    return (
        <div className={cx(style.search, { [style.active]: search })}>
            <i className={cx("fad fa-search fa-md", style.icon)} aria-hidden="true" />
            <input
                type="text"
                placeholder="Search Layers"
                value={search}
                ref={ref}
                onChange={(event) => setSearch(event.target.value)}
                {...focusProps}
                {...keyboardProps}
            />
            {search && (
                <FocusRing focusRingClass={style.clearFocus}>
                    <button className={cx(style.clearSearch, style.icon)} {...pressProps}>
                        <i className="fad fa-times-circle fa-sm" />
                    </button>
                </FocusRing>
            )}
        </div>
    );
}
