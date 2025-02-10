use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_814842370: FileType = FileType {
    file_format: &FileFormat {
        id: 814_842_370,
        source_type: SourceType::Iana,
        name: "vnd.yamaha.smaf-phrase",
        extensions: &[],
        media_types: &["application/vnd.yamaha.smaf-phrase"],
        signatures: &[],
        related_formats: &[],
    },
};
