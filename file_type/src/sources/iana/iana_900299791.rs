use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_900299791: FileType = FileType {
    file_format: &FileFormat {
        id: 900_299_791,
        source_type: SourceType::Iana,
        name: "kpml-response+xml",
        extensions: &[],
        media_types: &["application/kpml-response+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
