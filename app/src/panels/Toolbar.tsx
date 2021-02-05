import React from "react";
import cx from "clsx";
import { useSettings } from "../context/settings";
import { useApi } from "../context/wasm-api";
import style from "./Toolbar.module.scss";
import ToolbarButton from "../widgets/ToolbarButton";
import ToolbarLink from "../widgets/ToolbarLink";

export default function Toolbar(): JSX.Element {
    const [settings, dispatchSettings] = useSettings();
    const [{ add_widget }] = useApi();
    const showUI = !settings.hideUserInterface;

    return (
        <div className={style.container}>
            <div className={style.group}>
                <div className={style.toolbar}>
                    <div className={style.title}>
                        Fluss <span className={style.tag}>beta</span>
                    </div>
                    {add_widget && (
                        <ToolbarButton onPress={() => add_widget("smiley")}>
                            <i className="fad fa-smile-plus fa-lg" />
                        </ToolbarButton>
                    )}
                </div>
                <div className={style.toolbar} hidden>
                    <button className={style.button}>
                        <i className="fad fa-undo fa-lg" />
                    </button>
                    <button className={style.button}>
                        <i className="fad fa-redo fa-lg" />
                    </button>
                </div>
                <div className={style.toolbar} hidden>
                    <div className={style.button}>
                        <i className="fad fa-spinner-third fa-spin fa-lg" />
                    </div>
                    {showUI && <span>My_Design_File.eukolia</span>}
                </div>
            </div>
            <div className={style.group}>
                {showUI && (
                    <div className={style.toolbar} hidden>
                        <div className={style.button}>
                            <i className="fad fa-user-secret fa-lg" />
                        </div>
                        <div className={style.button}>
                            <i className="fad fa-user-injured fa-lg" />
                        </div>
                        <div className={style.button}>
                            <i className="fad fa-user-alien fa-lg" />
                        </div>
                        <div className={style.button}>
                            <i className="fad fa-user-astronaut fa-lg" />
                        </div>
                    </div>
                )}
                {showUI && (
                    <div className={style.toolbar}>
                        <div className={style.button} hidden>
                            <i className="fad fa-sliders-v fa-lg" />
                        </div>
                        <ToolbarLink
                            href="https://docs.eukolia.design"
                            target="_blank"
                            rel="noreferrer"
                        >
                            <i className="fad fa-book fa-lg" />
                        </ToolbarLink>
                        <ToolbarLink
                            href="https://twitter.com/intent/tweet\?text=Hi%20@EukoliaDesign"
                            target="_blank"
                            rel="noreferrer"
                        >
                            <i className="fad fa-comment-alt-smile fa-lg" />
                        </ToolbarLink>
                    </div>
                )}
                <div className={cx(style.toolbar, { [style.invert]: !showUI })}>
                    <ToolbarButton onPress={() => dispatchSettings({ type: "toggleUi" })}>
                        {showUI ? (
                            <i className="fad fa-window-frame-open fa-lg" />
                        ) : (
                            <i className="fad fa-window-frame fa-lg" />
                        )}
                    </ToolbarButton>
                </div>
            </div>
        </div>
    );
}

// TODO import widgets
