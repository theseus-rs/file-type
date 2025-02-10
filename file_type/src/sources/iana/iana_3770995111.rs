use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3770995111: FileType = FileType {
    file_format: &FileFormat {
        id: 3_770_995_111,
        source_type: SourceType::Iana,
        name: "vnd.ecowin.fileupdate",
        extensions: &[],
        media_types: &["application/vnd.ecowin.fileupdate"],
        signatures: &[],
        related_formats: &[],
    },
};
