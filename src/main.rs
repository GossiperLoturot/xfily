use std::io::Cursor;
use yew::prelude::*;

#[derive(Debug, Clone)]
enum Page {
    Home,
    ConvertImage,
}

#[function_component]
fn App() -> Html {
    let current_page = use_state(|| Page::Home);
    let navigate_factory = |page: Page| {
        let current_page = current_page.clone();
        move |_| current_page.set(page.clone())
    };

    html! {
        <div>
            <div>
                <button class="mx-3" onclick={navigate_factory(Page::Home)}>{"ホーム"}</button>
                <button class="mx-3" onclick={navigate_factory(Page::ConvertImage)}>{"画像変換"}</button>
            </div>

            {
                match *current_page {
                    Page::Home => html! { <Home />},
                    Page::ConvertImage => html! { <ConvertImage /> },
                }
            }
        </div>
    }
}

#[function_component]
fn Home() -> Html {
    html! {
        <div class="w-full">
            <div class="mx-auto mt-[10rem] w-[1280px]">
                <h1 class="mb-[2.5rem] text-center text-9xl">{"xfily"}</h1>
                <p class="mb-[10rem] text-center text-xl">{"ブラウザ上で完全に動作するファイルのコーデックス変換サービス"}</p>

                <div class="mb-[5rem] flex gap-[10rem]">
                    <div>
                        <p class="mb-3 text-2xl text-sky-600">{"スタンドアロン"}</p>
                        <p>{"クライアントサイドで完全に動作するように設計されているので, すべての処理をローカルで行うことができます"}</p>
                    </div>

                    <div>
                        <p class="mb-3 text-2xl text-rose-600">{"パフォーマンス"}</p>
                        <p>{"ファイルの変換処理にWebAssemblyを利用することで, パフォーマンスを向上させています"}</p>
                    </div>

                    <div>
                        <p class="mb-3 text-2xl text-emerald-600">{"プライバシー"}</p>
                        <p>{"ファイルはサーバにアップロードされることがないので, 安全に利用することができます"}</p>
                    </div>
                </div>

                <p class="mb-6 text-4xl">{"機能"}</p>
                <ul class="mb-[5rem]">
                    <li class="mb-6">
                        <p class="mb-1">{"ZIPや7ZIP, RARなど, N種類のコーデックスに対応したファイルとディレクトリの圧縮と展開"}</p>
                        <p class="text-gray-400">{"対応コーデックス: ZIP, 7ZIP, RAR, Z, GZIP, BZIP2, XZ, LZMA, LZOP, ZSTD"}</p>
                    </li>
                    <li class="mb-6">
                        <p class="mb-1">{"PNGやJPEG, BMPなど, N種類のコーデックスに対応した画像ファイルの読み込みと書き込み"}</p>
                        <p class="text-gray-400">{"対応コーデックス: PNG, JPEG, GIF, BMP, ICO, TIFF, WEBP, AVIF, PNM, DDS, TGA, OPENEXR, FARBFELD"}</p>
                    </li>
                    <li class="mb-6">
                        <p class="mb-1">{"WAVやFLAC, MP3など, N種類のコーデックスに対応した音声ファイルの読み込みと書き込み"}</p>
                        <p class="text-gray-400">{"対応コーデックス: -"}</p>
                    </li>
                    <li class="mb-6">
                        <p class="mb-1">{"MOVやAVI, MP4など, N種類のコーデックスに対応した映像ファイルの読み込みと書き込み"}</p>
                        <p class="text-gray-400">{"対応コーデックス: -"}</p>
                    </li>
                    <li class="mb-6">
                        <p class="mb-1">{"AESやRC4, RSAなど, N種類のコーデックスに対応したファイルの暗号化と復号化"}</p>
                        <p class="text-gray-400">{"対応コーデックス: BASE64, BASE64URL, BLOWFISH, CAST, CAST5, DES, GOST89, IDEA, RC2, RC4, RC5, AES"}</p>
                    </li>
                </ul>
            </div>
        </div>
    }
}

#[function_component]
fn ConvertImage() -> Html {
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
        <div>
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
                <span>{"output"}</span>
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

fn main() {
    yew::Renderer::<App>::new().render();
}
