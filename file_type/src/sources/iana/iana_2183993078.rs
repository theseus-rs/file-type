use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2183993078: FileType = FileType {
    file_format: &FileFormat {
        id: 2_183_993_078,
        source_type: SourceType::Iana,
        name: "vnd.uplanet.bearer-choice",
        extensions: &[],
        media_types: &["application/vnd.uplanet.bearer-choice"],
        signatures: &[],
        related_formats: &[],
    },
};
