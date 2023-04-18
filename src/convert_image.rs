use std::io::Cursor;
use yew::prelude::*;

#[function_component]
pub fn ConvertImage() -> Html {
    let task_handle = use_state(|| None);
    let input_type = use_state(|| String::new());
    let output_type = use_state(|| String::new());
    let metadata_handle = use_state(|| String::new());
    let url_handle = use_state(|| String::new());

    let input_type_onchange = {
        let input_type = input_type.clone();
        move |e: Event| {
            let target = e.target_dyn_into::<web_sys::HtmlSelectElement>().unwrap();
            input_type.set(target.value());
        }
    };

    let output_type_onchange = {
        let output_type = output_type.clone();
        move |e: Event| {
            let target = e.target_dyn_into::<web_sys::HtmlSelectElement>().unwrap();
            output_type.set(target.value());
        }
    };

    let input_image_onchange = {
        let task_handle = task_handle.clone();
        let metadata_handle = metadata_handle.clone();
        let url_handle = url_handle.clone();
        let input_type = input_type.clone();
        let output_type = output_type.clone();

        move |e: Event| {
            let target = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            let file = target.files().unwrap().get(0).unwrap();
            let file = gloo::file::File::from(file);

            let metadata_handle = metadata_handle.clone();
            let url_handle = url_handle.clone();
            let input_type = input_type.clone();
            let output_type = output_type.clone();

            let task = gloo::file::callbacks::read_as_bytes(&file, move |context| {
                let buf = context.unwrap();

                let reader = Cursor::new(buf);
                let input_type = match (*input_type).as_str() {
                    "PNG" => image::ImageFormat::Png,
                    "JPEG" => image::ImageFormat::Jpeg,
                    "WEBP" => image::ImageFormat::WebP,
                    "BMP" => image::ImageFormat::Bmp,
                    _ => unreachable!(),
                };
                let img = image::load(reader, input_type).unwrap();
                let (w, h, profile) = (img.width(), img.height(), img.color());
                metadata_handle.set(format!("{:?} x {:?} | {:?}", w, h, profile));

                let mut writer = Cursor::new(vec![]);
                let output_type = match (*output_type).as_str() {
                    "PNG" => image::ImageFormat::Png,
                    "JPEG" => image::ImageFormat::Jpeg,
                    "WEBP" => image::ImageFormat::WebP,
                    "BMP" => image::ImageFormat::Bmp,
                    _ => unreachable!(),
                };
                img.write_to(&mut writer, output_type).unwrap();
                let buf = gloo::file::Blob::new(writer.get_ref().as_slice());
                let url = web_sys::Url::create_object_url_with_blob(&buf.into()).unwrap();
                url_handle.set(url);
            });
            task_handle.set(Some(task));
        }
    };

    html! {
        <div class="p-5">
            <div>
                <span class="mx-3">{"input"}</span>
                <select class="mx-3" onchange={input_type_onchange} value={(*input_type).clone()}>
                    <option>{"JPEG"}</option>
                    <option>{"PNG"}</option>
                    <option>{"WEBP"}</option>
                    <option>{"BMP"}</option>
                </select>
                <input class="mx-3" type="file" onchange={input_image_onchange} />
                <span class="mx-3">{(*metadata_handle).clone()}</span>
            </div>

            <div>
                <span class="mx-3">{"output"}</span>
                <select class="mx-3" onchange={output_type_onchange} value={(*output_type).clone()}>
                    <option>{"JPEG"}</option>
                    <option>{"PNG"}</option>
                    <option>{"WEBP"}</option>
                    <option>{"BMP"}</option>
                </select>
                <img src={(*url_handle).clone()} />
            </div>
        </div>
    }
}
