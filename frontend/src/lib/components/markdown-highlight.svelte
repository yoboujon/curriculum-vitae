<script>
  import "$lib/css/markdown.css";
  import "highlight.js/styles/base16/atelier-estuary-light.min.css";

  import hljs from "highlight.js/lib/core";
  // Language support static import
  import javascript from "highlight.js/lib/languages/javascript";
  import bash from "highlight.js/lib/languages/bash";
  import csharp from "highlight.js/lib/languages/csharp"
  import c from "highlight.js/lib/languages/c";
  import cpp from "highlight.js/lib/languages/cpp";
  import armasm from "highlight.js/lib/languages/armasm";
  import arduino from "highlight.js/lib/languages/arduino";
  import cmake from "highlight.js/lib/languages/cmake";
  import css from "highlight.js/lib/languages/css";
  import json from "highlight.js/lib/languages/json";
  import java from "highlight.js/lib/languages/java";
  import latex from "highlight.js/lib/languages/latex";
  import matlab from "highlight.js/lib/languages/matlab";
  import nginx from "highlight.js/lib/languages/nginx";
  import php from "highlight.js/lib/languages/php";
  import python from "highlight.js/lib/languages/python";
  import rust from "highlight.js/lib/languages/rust";
  import sql from "highlight.js/lib/languages/sql";
  import shell from "highlight.js/lib/languages/shell";
  import vhdl from "highlight.js/lib/languages/vhdl";
  import xml from "highlight.js/lib/languages/xml";
  // Addons
  import svelte from "highlightjs-svelte";

  // Registering all languages
  hljs.registerLanguage("javascript", javascript);
  hljs.registerLanguage("bash", bash);
  hljs.registerLanguage("csharp",csharp);
  hljs.registerLanguage("c", c);
  hljs.registerLanguage("cpp", cpp);
  hljs.registerLanguage("armasm", armasm);
  hljs.registerLanguage("arduino", arduino);
  hljs.registerLanguage("cmake", cmake);
  hljs.registerLanguage("css", css);
  hljs.registerLanguage("json", json);
  hljs.registerLanguage("java", java);
  hljs.registerLanguage("latex", latex);
  hljs.registerLanguage("matlab", matlab);
  hljs.registerLanguage("nginx", nginx);
  hljs.registerLanguage("php", php);
  hljs.registerLanguage("python", python);
  hljs.registerLanguage("rust",rust);
  hljs.registerLanguage("sql",sql);
  hljs.registerLanguage("shell", shell);
  hljs.registerLanguage("vhdl", vhdl);
  hljs.registerLanguage("xml", xml);
  // Addons
  svelte(hljs);

  // Exporting values
  export let lang;
  export let text;
  let littleLanguage;
  let html = text;

  if(hljs.getLanguage(lang) != undefined)
  {
    html = hljs.highlight(text, { language: lang }).value;
  }

  function showLang(visible) {
    if (visible) littleLanguage.style.visibility = "visible";
    else littleLanguage.style.visibility = "hidden";
  }
</script>

<div
  class="markdown-codeblock"
  on:mouseover={() => showLang(true)}
  on:mouseleave={() => showLang(false)}
  on:focus={() => true}
  role="link"
  tabindex="0"
>
  <div class="little-language" bind:this={littleLanguage}>{lang}</div>
  <pre>{@html html}</pre>
</div>
