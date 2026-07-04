fn main() {
    // Эта команда говорит компилятору: 
    // "Возьми файл icon.rc, найди в нём ссылку на icon.ico 
    // и вшей этот ICO внутрь итогового exe-файла"
    embed_resource::compile("assets/icon.rc", embed_resource::NONE);
}