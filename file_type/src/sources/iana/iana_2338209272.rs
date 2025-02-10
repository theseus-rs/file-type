use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2338209272: FileType = FileType {
    file_format: &FileFormat {
        id: 2_338_209_272,
        source_type: SourceType::Iana,
        name: "G722",
        extensions: &[],
        media_types: &["audio/G722"],
        signatures: &[],
        related_formats: &[],
    },
};
