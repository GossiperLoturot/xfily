import { Component } from "solid-js";

export const ConvertImage: Component = () => {
  return (
    <div class="p-5">
      <div>
        <span class="mx-3">{"input"}</span>
        <select class="mx-3">
          <option>{"JPEG"}</option>
          <option>{"PNG"}</option>
          <option>{"WEBP"}</option>
          <option>{"BMP"}</option>
        </select>
        <input class="mx-3" type="file" />
      </div>

      <div>
        <span class="mx-3">{"output"}</span>
        <select class="mx-3">
          <option>{"JPEG"}</option>
          <option>{"PNG"}</option>
          <option>{"WEBP"}</option>
          <option>{"BMP"}</option>
        </select>
      </div>
    </div>
  );
};
