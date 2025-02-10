use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1235227822: FileType = FileType {
    file_format: &FileFormat {
        id: 1_235_227_822,
        source_type: SourceType::Iana,
        name: "vnd.3gpp2.tcap",
        extensions: &[],
        media_types: &["application/vnd.3gpp2.tcap"],
        signatures: &[],
        related_formats: &[],
    },
};
