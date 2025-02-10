use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3241319755: FileType = FileType {
    file_format: &FileFormat {
        id: 3_241_319_755,
        source_type: SourceType::Iana,
        name: "vnd.radisys.msml+xml",
        extensions: &[],
        media_types: &["application/vnd.radisys.msml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
