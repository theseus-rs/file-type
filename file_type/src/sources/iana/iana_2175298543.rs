use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2175298543: FileType = FileType {
    file_format: &FileFormat {
        id: 2_175_298_543,
        source_type: SourceType::Iana,
        name: "cybercash",
        extensions: &[],
        media_types: &["application/cybercash"],
        signatures: &[],
        related_formats: &[],
    },
};
