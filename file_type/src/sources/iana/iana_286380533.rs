use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_286380533: FileType = FileType {
    file_format: &FileFormat {
        id: 286_380_533,
        source_type: SourceType::Iana,
        name: "vnd.oma.dcdc",
        extensions: &[],
        media_types: &["application/vnd.oma.dcdc"],
        signatures: &[],
        related_formats: &[],
    },
};
