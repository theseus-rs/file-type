use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_288405: FileType = FileType {
    file_format: &FileFormat {
        id: 288_405,
        source_type: SourceType::Wikidata,
        name: "hOCR",
        extensions: &["hocr", "html"],
        media_types: &["text/html", "text/vnd.hocr+html"],
        signatures: &[],
        related_formats: &[],
    },
};
