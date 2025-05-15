use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2392801665: FileType = FileType {
    file_format: &FileFormat {
        id: 2_392_801_665,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.seal-app-comm-requirements-info+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.seal-app-comm-requirements-info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
