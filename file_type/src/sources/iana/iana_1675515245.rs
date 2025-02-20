use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1675515245: FileType = FileType {
    file_format: &FileFormat {
        id: 1_675_515_245,
        source_type: SourceType::Iana,
        name: "vnd.blueice.multipass",
        extensions: &[],
        media_types: &["application/vnd.blueice.multipass"],
        signatures: &[],
        related_formats: &[],
    },
};
