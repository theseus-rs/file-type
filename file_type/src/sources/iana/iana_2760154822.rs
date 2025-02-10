use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2760154822: FileType = FileType {
    file_format: &FileFormat {
        id: 2_760_154_822,
        source_type: SourceType::Iana,
        name: "vnd.epson.esf",
        extensions: &[],
        media_types: &["application/vnd.epson.esf"],
        signatures: &[],
        related_formats: &[],
    },
};
