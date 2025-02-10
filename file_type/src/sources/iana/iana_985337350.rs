use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_985337350: FileType = FileType {
    file_format: &FileFormat {
        id: 985_337_350,
        source_type: SourceType::Iana,
        name: "vnd.loom",
        extensions: &[],
        media_types: &["application/vnd.loom"],
        signatures: &[],
        related_formats: &[],
    },
};
