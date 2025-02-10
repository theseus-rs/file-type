use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_78756907: FileType = FileType {
    file_format: &FileFormat {
        id: 78_756_907,
        source_type: SourceType::Iana,
        name: "vnd.gs-gdl",
        extensions: &[],
        media_types: &["model/vnd.gs-gdl"],
        signatures: &[],
        related_formats: &[],
    },
};
