use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4039823969: FileType = FileType {
    file_format: &FileFormat {
        id: 4_039_823_969,
        source_type: SourceType::Iana,
        name: "vnd.oipf.spdlist+xml",
        extensions: &[],
        media_types: &["application/vnd.oipf.spdlist+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
