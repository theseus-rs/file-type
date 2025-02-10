use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3315716607: FileType = FileType {
    file_format: &FileFormat {
        id: 3_315_716_607,
        source_type: SourceType::Iana,
        name: "vnd.enphase.envoy",
        extensions: &[],
        media_types: &["application/vnd.enphase.envoy"],
        signatures: &[],
        related_formats: &[],
    },
};
