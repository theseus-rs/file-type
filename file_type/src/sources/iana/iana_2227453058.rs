use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2227453058: FileType = FileType {
    file_format: &FileFormat {
        id: 2_227_453_058,
        source_type: SourceType::Iana,
        name: "vnd.ericsson.quickcall",
        extensions: &[],
        media_types: &["application/vnd.ericsson.quickcall"],
        signatures: &[],
        related_formats: &[],
    },
};
