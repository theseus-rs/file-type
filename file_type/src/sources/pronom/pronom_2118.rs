use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2118: FileType = FileType {
    file_format: &FileFormat {
        id: 2_118,
        source_type: SourceType::Pronom,
        name: "Broderbund The Print Shop/PrintMaster/American Greetings Project",
        extensions: &[
            "ban", "bro", "biz", "cal", "car", "cer", "env", "fax", "sig", "cft", "hcr", "lbl",
            "let", "nws", "not", "pcr", "php", "tsh", "web", "sti",
        ],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
