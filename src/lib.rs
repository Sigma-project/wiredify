//! # wiredify
//! バ行のカタカナを"ヴァ"～”ヴォ”に変換する

pub fn wiredify (s: String) -> String {
    let result1: String = s.replace("バ", "ヴァ");
    let result2: String = result1.replace("ビ", "ヴィ");
    let result3: String = result2.replace("ブ", "ヴ");
    let result4: String = result3.replace("ベ", "ヴェ");
    let result: String = result4.replace("ボ", "ヴォ");
    return result;
}