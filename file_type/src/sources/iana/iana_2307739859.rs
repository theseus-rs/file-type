use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2307739859: FileType = FileType {
    file_format: &FileFormat {
        id: 2_307_739_859,
        source_type: SourceType::Iana,
        name: "dsr-es202050",
        extensions: &[],
        media_types: &["audio/dsr-es202050"],
        signatures: &[],
        related_formats: &[],
    },
};
