use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2265585215: FileType = FileType {
    file_format: &FileFormat {
        id: 2_265_585_215,
        source_type: SourceType::Iana,
        name: "trig",
        extensions: &[],
        media_types: &["application/trig"],
        signatures: &[],
        related_formats: &[],
    },
};
