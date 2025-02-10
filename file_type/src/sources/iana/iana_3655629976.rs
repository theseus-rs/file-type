use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3655629976: FileType = FileType {
    file_format: &FileFormat {
        id: 3_655_629_976,
        source_type: SourceType::Iana,
        name: "vnd.triscape.mxs",
        extensions: &[],
        media_types: &["application/vnd.triscape.mxs"],
        signatures: &[],
        related_formats: &[],
    },
};
