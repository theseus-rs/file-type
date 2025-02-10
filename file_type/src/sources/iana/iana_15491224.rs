use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_15491224: FileType = FileType {
    file_format: &FileFormat {
        id: 15_491_224,
        source_type: SourceType::Iana,
        name: "vnd.gmx - DEPRECATED",
        extensions: &[],
        media_types: &["application/vnd.gmx"],
        signatures: &[],
        related_formats: &[],
    },
};
