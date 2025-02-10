use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1886613558: FileType = FileType {
    file_format: &FileFormat {
        id: 1_886_613_558,
        source_type: SourceType::Iana,
        name: "vnd.ubisoft.webplayer",
        extensions: &[],
        media_types: &["application/vnd.ubisoft.webplayer"],
        signatures: &[],
        related_formats: &[],
    },
};
