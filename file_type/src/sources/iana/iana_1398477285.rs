use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1398477285: FileType = FileType {
    file_format: &FileFormat {
        id: 1_398_477_285,
        source_type: SourceType::Iana,
        name: "vnd.firemonkeys.cloudcell",
        extensions: &[],
        media_types: &["application/vnd.firemonkeys.cloudcell"],
        signatures: &[],
        related_formats: &[],
    },
};
