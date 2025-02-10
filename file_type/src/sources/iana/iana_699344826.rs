use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_699344826: FileType = FileType {
    file_format: &FileFormat {
        id: 699_344_826,
        source_type: SourceType::Iana,
        name: "vnd.Quark.QuarkXPress",
        extensions: &[],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};
