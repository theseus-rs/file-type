use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_694257130: FileType = FileType {
    file_format: &FileFormat {
        id: 694_257_130,
        source_type: SourceType::Iana,
        name: "vnd.ficlab.flt",
        extensions: &[],
        media_types: &["text/vnd.ficlab.flt"],
        signatures: &[],
        related_formats: &[],
    },
};
