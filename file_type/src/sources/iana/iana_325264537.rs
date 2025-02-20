use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_325264537: FileType = FileType {
    file_format: &FileFormat {
        id: 325_264_537,
        source_type: SourceType::Iana,
        name: "vnd.radisys.msml-conf+xml",
        extensions: &[],
        media_types: &["application/vnd.radisys.msml-conf+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
