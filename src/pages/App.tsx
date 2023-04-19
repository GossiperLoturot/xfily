import { Component, Match, Switch, createSignal } from "solid-js";
import { Home } from "./Home";
import { ConvertImage } from "./ConvertImage";

const App: Component = () => {
  const [page, setPage] = createSignal("HOME");

  return (
    <>
      <div class="sticky top-0 border-b bg-white p-3">
        <button class="mx-3" onClick={() => setPage("HOME")}>
          ホーム
        </button>
        <button class="mx-3" onClick={() => setPage("CONV_COMPRESS")}>
          圧縮変換
        </button>
        <button class="mx-3" onClick={() => setPage("CONV_IMAGE")}>
          画像変換
        </button>
        <button class="mx-3" onClick={() => setPage("CONV_AUDIO")}>
          音声変換
        </button>
        <button class="mx-3" onClick={() => setPage("CONV_VIDEO")}>
          映像変換
        </button>
        <button class="mx-3" onClick={() => setPage("CONV_ENCRYPT")}>
          暗号変換
        </button>
      </div>

      <Switch>
        <Match when={page() == "HOME"}>
          <Home />
        </Match>
        <Match when={page() == "CONV_IMAGE"}>
          <ConvertImage />
        </Match>
      </Switch>
    </>
  );
};

export default App;
