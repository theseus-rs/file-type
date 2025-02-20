use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3628822850: FileType = FileType {
    file_format: &FileFormat {
        id: 3_628_822_850,
        source_type: SourceType::Iana,
        name: "G726-24",
        extensions: &[],
        media_types: &["audio/G726-24"],
        signatures: &[],
        related_formats: &[],
    },
};
