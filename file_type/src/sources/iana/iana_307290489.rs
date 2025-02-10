use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_307290489: FileType = FileType {
    file_format: &FileFormat {
        id: 307_290_489,
        source_type: SourceType::Iana,
        name: "vnd.cisco.nse",
        extensions: &[],
        media_types: &["audio/vnd.cisco.nse"],
        signatures: &[],
        related_formats: &[],
    },
};
