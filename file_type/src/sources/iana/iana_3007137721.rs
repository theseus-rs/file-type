use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3007137721: FileType = FileType {
    file_format: &FileFormat {
        id: 3_007_137_721,
        source_type: SourceType::Iana,
        name: "mbms-reception-report+xml",
        extensions: &[],
        media_types: &["application/mbms-reception-report+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
