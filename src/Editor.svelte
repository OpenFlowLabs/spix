<script lang="ts">
  import { onMount } from "svelte";
  import "grapesjs/dist/css/grapes.min.css";

  import grapesjs from "grapesjs";
  import toastr from "toastr";
  import gjs_preset_webpage from "grapesjs-preset-webpage";
  import gjs_blocks_basic from "grapesjs-blocks-basic";
  import gjs_plugin_forms from "grapesjs-plugin-forms";
  import gjs_component_countdown from "grapesjs-component-countdown";
  import gjs_plugin_export from "grapesjs-plugin-export";
  import gjs_tabs from "grapesjs-tabs";
  import gjs_custom_code from "grapesjs-custom-code";
  import gjs_touch from "grapesjs-touch";
  import gjs_tooltip from "grapesjs-tooltip";
  import gjs_tui_image_editor from "grapesjs-tui-image-editor";
  import gjs_typed from "grapesjs-typed";
  import gjs_style from "grapesjs-style-bg";
  import { load_file, save_file, Site } from "tauri-plugin-site-api";

  export let site: Site;

  let editor: grapesjs.Editor;

  interface TauriStorageOptions {
    site: Site;
  }

  const tauriStoragePlugin = function (editor: grapesjs.Editor) {
    // As sessionStorage is not an asynchronous API,
    // the `async` keyword could be skipped
    editor.Storage.add("tauri", {
      async load(options: TauriStorageOptions) {
        try {
          let project_data = site.files.filter(
            (value) => value.asset_type == "ProjectData"
          );
          if (project_data.length == 0) {
            return {"pages": [{"component": "<div>Initial content</div>"}]};
          }
          return JSON.parse(
            await load_file(options.site.name, project_data[0].digest.value)
          );
        } catch (e) {
          console.log(e);
          return {};
        }
      },

      async store(data, options: TauriStorageOptions) {
        try {
          //TODO Method to save all files at once
          let html = editor.getHtml();
          let css = editor.getCss().toString();
          let js = editor.getJs();
          let localsite = await save_file(options.site, JSON.stringify(data), "ProjectData");
          localsite = await save_file(localsite, html, "Html");
          localsite = await save_file(localsite, css, "Css");
          localsite = await save_file(localsite, js, "Script");
        } catch (e) {
          console.log(e);
        }
      },
    });
  };

  onMount(() => {
    editor = grapesjs.init({
      height: "100vh",
      container: "#gjs",
      components: "<h1>Hello World Component!</h1>",
      showOffsets: true,
      selectorManager: true,
      styleManager: {
        sectors: [
          {
            name: "General",
            properties: [
              {
                extend: "float",
                type: "radio",
                default: "none",
                options: [
                  { value: "none", className: "fa fa-times" },
                  { value: "left", className: "fa fa-align-left" },
                  { value: "right", className: "fa fa-align-right" },
                ],
              },
              "display",
              { extend: "position", type: "select" },
              "top",
              "right",
              "left",
              "bottom",
            ],
          },
          {
            name: "Dimension",
            open: false,
            properties: [
              "width",
              {
                id: "flex-width",
                type: "integer",
                name: "Width",
                units: ["px", "%"],
                property: "flex-basis",
                toRequire: 1,
              },
              "height",
              "max-width",
              "min-height",
              "margin",
              "padding",
            ],
          },
          {
            name: "Typography",
            open: false,
            properties: [
              "font-family",
              "font-size",
              "font-weight",
              "letter-spacing",
              "color",
              "line-height",
              {
                extend: "text-align",
                options: [
                  { id: "left", label: "Left", className: "fa fa-align-left" },
                  {
                    id: "center",
                    label: "Center",
                    className: "fa fa-align-center",
                  },
                  {
                    id: "right",
                    label: "Right",
                    className: "fa fa-align-right",
                  },
                  {
                    id: "justify",
                    label: "Justify",
                    className: "fa fa-align-justify",
                  },
                ],
              },
              {
                property: "text-decoration",
                type: "radio",
                default: "none",
                options: [
                  { id: "none", label: "None", className: "fa fa-times" },
                  {
                    id: "underline",
                    label: "underline",
                    className: "fa fa-underline",
                  },
                  {
                    id: "line-through",
                    label: "Line-through",
                    className: "fa fa-strikethrough",
                  },
                ],
              },
              "text-shadow",
            ],
          },
          {
            name: "Decorations",
            open: false,
            properties: [
              "opacity",
              "border-radius",
              "border",
              "box-shadow",
              "background", // { id: 'background-bg', property: 'background', type: 'bg' }
            ],
          },
          {
            name: "Extra",
            open: false,
            buildProps: ["transition", "perspective", "transform"],
          },
          {
            name: "Flex",
            open: false,
            properties: [
              {
                name: "Flex Container",
                property: "display",
                type: "select",
                defaults: "block",
                list: [
                  { value: "block", name: "Disable" },
                  { value: "flex", name: "Enable" },
                ],
              },
              {
                name: "Flex Parent",
                property: "label-parent-flex",
                type: "integer",
              },
              {
                name: "Direction",
                property: "flex-direction",
                type: "radio",
                defaults: "row",
                list: [
                  {
                    value: "row",
                    name: "Row",
                    className: "icons-flex icon-dir-row",
                    title: "Row",
                  },
                  {
                    value: "row-reverse",
                    name: "Row reverse",
                    className: "icons-flex icon-dir-row-rev",
                    title: "Row reverse",
                  },
                  {
                    value: "column",
                    name: "Column",
                    title: "Column",
                    className: "icons-flex icon-dir-col",
                  },
                  {
                    value: "column-reverse",
                    name: "Column reverse",
                    title: "Column reverse",
                    className: "icons-flex icon-dir-col-rev",
                  },
                ],
              },
              {
                name: "Justify",
                property: "justify-content",
                type: "radio",
                defaults: "flex-start",
                list: [
                  {
                    value: "flex-start",
                    className: "icons-flex icon-just-start",
                    title: "Start",
                  },
                  {
                    value: "flex-end",
                    title: "End",
                    className: "icons-flex icon-just-end",
                  },
                  {
                    value: "space-between",
                    title: "Space between",
                    className: "icons-flex icon-just-sp-bet",
                  },
                  {
                    value: "space-around",
                    title: "Space around",
                    className: "icons-flex icon-just-sp-ar",
                  },
                  {
                    value: "center",
                    title: "Center",
                    className: "icons-flex icon-just-sp-cent",
                  },
                ],
              },
              {
                name: "Align",
                property: "align-items",
                type: "radio",
                defaults: "center",
                list: [
                  {
                    value: "flex-start",
                    title: "Start",
                    className: "icons-flex icon-al-start",
                  },
                  {
                    value: "flex-end",
                    title: "End",
                    className: "icons-flex icon-al-end",
                  },
                  {
                    value: "stretch",
                    title: "Stretch",
                    className: "icons-flex icon-al-str",
                  },
                  {
                    value: "center",
                    title: "Center",
                    className: "icons-flex icon-al-center",
                  },
                ],
              },
              {
                name: "Flex Children",
                property: "label-parent-flex",
                type: "integer",
              },
              {
                name: "Order",
                property: "order",
                type: "integer",
                defaults: 0,
                min: 0,
              },
              {
                name: "Flex",
                property: "flex",
                type: "composite",
                properties: [
                  {
                    name: "Grow",
                    property: "flex-grow",
                    type: "integer",
                    defaults: 0,
                    min: 0,
                  },
                  {
                    name: "Shrink",
                    property: "flex-shrink",
                    type: "integer",
                    defaults: 0,
                    min: 0,
                  },
                  {
                    name: "Basis",
                    property: "flex-basis",
                    type: "integer",
                    units: ["px", "%", ""],
                    unit: "",
                    defaults: "auto",
                  },
                ],
              },
              {
                name: "Align",
                property: "align-self",
                type: "radio",
                defaults: "auto",
                list: [
                  {
                    value: "auto",
                    name: "Auto",
                  },
                  {
                    value: "flex-start",
                    title: "Start",
                    className: "icons-flex icon-al-start",
                  },
                  {
                    value: "flex-end",
                    title: "End",
                    className: "icons-flex icon-al-end",
                  },
                  {
                    value: "stretch",
                    title: "Stretch",
                    className: "icons-flex icon-al-str",
                  },
                  {
                    value: "center",
                    title: "Center",
                    className: "icons-flex icon-al-center",
                  },
                ],
              },
            ],
          },
        ],
      },
      plugins: [
        gjs_blocks_basic,
        gjs_plugin_forms,
        gjs_component_countdown,
        gjs_plugin_export,
        gjs_tabs,
        gjs_custom_code,
        gjs_touch,
        gjs_tooltip,
        gjs_tui_image_editor,
        gjs_typed,
        gjs_style,
        gjs_preset_webpage,
        tauriStoragePlugin,
      ],
      pluginsOpts: {
        "gjs-blocks-basic": { flexGrid: true },
        "grapesjs-tui-image-editor": {
          script: [
            // 'https://cdnjs.cloudflare.com/ajax/libs/fabric.js/1.6.7/fabric.min.js',
            "https://uicdn.toast.com/tui.code-snippet/v1.5.2/tui-code-snippet.min.js",
            "https://uicdn.toast.com/tui-color-picker/v2.2.7/tui-color-picker.min.js",
            "https://uicdn.toast.com/tui-image-editor/v3.15.2/tui-image-editor.min.js",
          ],
          style: [
            "https://uicdn.toast.com/tui-color-picker/v2.2.7/tui-color-picker.min.css",
            "https://uicdn.toast.com/tui-image-editor/v3.15.2/tui-image-editor.min.css",
          ],
        },
        "grapesjs-tabs": {
          tabsBlock: { category: "Extra" },
        },
        "grapesjs-typed": {
          block: {
            category: "Extra",
            content: {
              type: "typed",
              "type-speed": 40,
              strings: ["Text row one", "Text row two", "Text row three"],
            },
          },
        },
      },
      storageManager: {
        type: "tauri",
        options: {
          tauri: { site: site },
        },
      },
    });

    var pn = editor.Panels;
    var cmdm = editor.Commands;

    // Update canvas-clear command
    cmdm.add("canvas-clear", function () {
      if (confirm("Are you sure to clean the canvas?")) {
        editor.runCommand("core:canvas-clear");
        setTimeout(function () {
          localStorage.clear();
        }, 0);
      }
    });

    // Add and beautify tooltips
    [
      ["sw-visibility", "Show Borders"],
      ["preview", "Preview"],
      ["undo", "Undo"],
      ["redo", "Redo"],
      ["gjs-open-import-webpage", "Import"],
      ["canvas-clear", "Clear canvas"],
    ].forEach(function (item) {
      pn.getButton("options", item[0]).set("attributes", {
        title: item[1],
        "data-tooltip-pos": "bottom",
      });
    });
    [
      ["open-sm", "Style Manager"],
      ["open-layers", "Layers"],
      ["open-blocks", "Blocks"],
    ].forEach(function (item) {
      pn.getButton("views", item[0]).set("attributes", {
        title: item[1],
        "data-tooltip-pos": "bottom",
      });
    });
    var titles = document.querySelectorAll("*[title]");

    ["fullscreen", "export-template"].forEach(function (item) {
      pn.removeButton("options", item);
    });

    for (var i = 0; i < titles.length; i++) {
      var el = titles[i];
      var title = el.getAttribute("title");
      title = title ? title.trim() : "";
      if (!title) break;
      el.setAttribute("data-tooltip", title);
      el.setAttribute("title", "");
    }

    // Store and load events
    editor.on("storage:load", function (e) {
      console.log("Loaded ", e);
    });
    editor.on("storage:store", function (e) {
      console.log("Stored ", e);
    });

    cmdm.add("save", async function (editor) {
      await editor.store({});
    });
    //@ts-ignored
    pn.addButton("options", {
      id: "save",
      className: "fa fa-save",
      command: () => {
        editor.runCommand("save");
      },
      attributes: {
        title: "Save to disk",
        "data-tooltip-pos": "bottom",
      },
    });
  });
</script>

<div id="gjs" />

<style type="text/css">
  .icons-flex {
    background-size: 70% 65% !important;
    height: 15px;
    width: 17px;
    opacity: 0.9;
  }

  .icon-dir-row {
    background: url("./img/flex-dir-row.png") no-repeat center;
  }

  .icon-dir-row-rev {
    background: url("./img/flex-dir-row-rev.png") no-repeat center;
  }

  .icon-dir-col {
    background: url("./img/flex-dir-col.png") no-repeat center;
  }

  .icon-dir-col-rev {
    background: url("./img/flex-dir-col-rev.png") no-repeat center;
  }

  .icon-just-start {
    background: url("./img/flex-just-start.png") no-repeat center;
  }

  .icon-just-end {
    background: url("./img/flex-just-end.png") no-repeat center;
  }

  .icon-just-sp-bet {
    background: url("./img/flex-just-sp-bet.png") no-repeat center;
  }

  .icon-just-sp-ar {
    background: url("./img/flex-just-sp-ar.png") no-repeat center;
  }

  .icon-just-sp-cent {
    background: url("./img/flex-just-sp-cent.png") no-repeat center;
  }

  .icon-al-start {
    background: url("./img/flex-al-start.png") no-repeat center;
  }

  .icon-al-end {
    background: url("./img/flex-al-end.png") no-repeat center;
  }

  .icon-al-str {
    background: url("./img/flex-al-str.png") no-repeat center;
  }

  .icon-al-center {
    background: url("./img/flex-al-center.png") no-repeat center;
  }

  [data-tooltip]::after {
    background: rgba(51, 51, 51, 0.9);
  }

  .gjs-pn-commands {
    min-height: 40px;
  }

  #gjs-sm-float {
    display: none;
  }

  .gjs-logo-version {
    background-color: #756467;
  }

  .gjs-pn-btn.gjs-pn-active {
    box-shadow: none;
  }

  .CodeMirror {
    min-height: 450px;
    margin-bottom: 8px;
  }

  .grp-handler-close {
    background-color: transparent;
    color: #ddd;
  }

  .grp-handler-cp-wrap {
    border-color: transparent;
  }
</style>
