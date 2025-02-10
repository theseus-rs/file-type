use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2742726075: FileType = FileType {
    file_format: &FileFormat {
        id: 2_742_726_075,
        source_type: SourceType::Iana,
        name: "vnd.moml+xml",
        extensions: &[],
        media_types: &["model/vnd.moml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
