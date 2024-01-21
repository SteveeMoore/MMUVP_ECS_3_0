use std::fs;

use crate::consts::FILE_OUTPUT_PATH;


pub fn clear_output_folder(){
    // Получаем список файлов и директорий внутри указанной папки
    let entries = fs::read_dir(FILE_OUTPUT_PATH).expect("Ошибка открытия дирректории вывода");

    for entry in entries {
        let entry = entry.expect("Ошибка проверки наличия файла");
        let path = entry.path();

        // Проверяем, является ли элемент файлом
        if path.is_file() {
            // Удаляем файл
            fs::remove_file(&path).expect("Ошибка удаления файла");
        }
    }
}