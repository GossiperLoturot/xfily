import { Component, createSignal } from "solid-js";
import init, { convert_image } from "../../wasm/pkg";

export const ConvertImage: Component = () => {
  const [getFile, setFile] = createSignal<File | null | undefined>();
  const [getInFormat, setInFormat] = createSignal("0");
  const [getOutFormat, setOutFormat] = createSignal("0");
  const [getUrl, setUrl] = createSignal<string | undefined>();

  const convert = async () => {
    await init();

    const file = getFile();
    if (!file) {
      console.error("not found files");
      return;
    }

    const reader = file.stream().getReader();
    const in_data = new Uint8Array(file.size);
    let count = 0;
    while (true) {
      let { done, value } = await reader.read();

      if (!value) break;
      in_data.set(value, count);
      count += value.length;

      if (done) break;
    }

    const out_data = convert_image(
      in_data,
      parseInt(getInFormat()),
      parseInt(getOutFormat())
    );
    setUrl(URL.createObjectURL(new Blob([out_data])));
  };

  return (
    <div class="p-5">
      <div>
        <span class="mx-3">INPUT</span>
        <select
          class="mx-3"
          value={getInFormat()}
          onChange={(e) => setInFormat(e.target.value)}
        >
          <option value={"0"}>GIF</option>
          <option value={"1"}>JPEG</option>
          <option value={"2"}>PNG</option>
          <option value={"3"}>PNM</option>
          <option value={"4"}>TGA</option>
          <option value={"5"}>TIFF</option>
          <option value={"6"}>WEBP</option>
          <option value={"7"}>BMP</option>
        </select>
        <input
          class="mx-3"
          type="file"
          onChange={(e) => setFile(e.target.files?.item(0))}
        />
      </div>

      <div>
        <button onClick={(_) => convert()}>変換</button>
      </div>

      <div>
        <span class="mx-3">OUTPUT</span>
        <select
          class="mx-3"
          value={getOutFormat()}
          onChange={(e) => setOutFormat(e.target.value)}
        >
          <option value={"0"}>GIF</option>
          <option value={"1"}>JPEG</option>
          <option value={"2"}>PNG</option>
          <option value={"3"}>PNM</option>
          <option value={"4"}>TGA</option>
          <option value={"5"}>TIFF</option>
          <option value={"6"}>WEBP (NOT SUPPORTED)</option>
          <option value={"7"}>BMP</option>
        </select>
      </div>

      <a href={getUrl()} download>
        DOWNLOAD
      </a>
    </div>
  );
};
