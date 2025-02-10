use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_89029185: FileType = FileType {
    file_format: &FileFormat {
        id: 89_029_185,
        source_type: SourceType::Wikidata,
        name: "American Greetings Project 10-23",
        extensions: &[
            "ban", "biz", "bro", "cal", "car", "cer", "cft", "env", "fax", "hcr", "lbl", "let",
            "not", "nws", "pcr", "php", "sig", "sti", "tsh", "web",
        ],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
