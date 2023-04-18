mod convert_image;

use crate::convert_image::ConvertImage;
use yew::prelude::*;

#[derive(Debug, Clone)]
enum Page {
    Home,
    ConvertCompression,
    ConvertImage,
    ConvertAudio,
    ConvertVideo,
    ConvertEncrypt,
}

#[function_component]
fn App() -> Html {
    let current_page = use_state(|| Page::Home);
    let navigate_factory = |page: Page| {
        let current_page = current_page.clone();
        move |_| current_page.set(page.clone())
    };

    html! {
        <>
            <div class="sticky top-0 bg-white border-b p-3">
                <button class="mx-3" onclick={navigate_factory(Page::Home)}>{"ホーム"}</button>
                <button class="mx-3" onclick={navigate_factory(Page::ConvertCompression)}>{"圧縮変換"}</button>
                <button class="mx-3" onclick={navigate_factory(Page::ConvertImage)}>{"画像変換"}</button>
                <button class="mx-3" onclick={navigate_factory(Page::ConvertAudio)}>{"音声変換"}</button>
                <button class="mx-3" onclick={navigate_factory(Page::ConvertVideo)}>{"映像変換"}</button>
                <button class="mx-3" onclick={navigate_factory(Page::ConvertEncrypt)}>{"暗号変換"}</button>
            </div>

            {
                match *current_page {
                    Page::Home => html! { <Home /> },
                    Page::ConvertCompression => html! { <Dummy /> },
                    Page::ConvertImage => html! { <ConvertImage /> },
                    Page::ConvertAudio => html! { <Dummy /> },
                    Page::ConvertVideo => html! { <Dummy /> },
                    Page::ConvertEncrypt => html! { <Dummy /> },
                }
            }
        </>
    }
}

#[function_component]
fn Home() -> Html {
    html! {
        <div class="mx-auto mt-[5rem] w-[1024px] px-[64px]">
            <h1 class="mb-[2.5rem] text-center text-9xl">{"xfily"}</h1>
            <p class="mb-[10rem] text-center text-xl">{"ブラウザ上で完全に動作するファイルのコーデックス変換サービス"}</p>

            <div class="mb-[5rem] flex gap-[5rem]">
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
    }
}

#[function_component]
fn Dummy() -> Html {
    html! {
        <div class="p-3">{"未実装"}</div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
