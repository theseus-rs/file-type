use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_520781800: FileType = FileType {
    file_format: &FileFormat {
        id: 520_781_800,
        source_type: SourceType::Iana,
        name: "vnd.nato.bindingdataobject+json",
        extensions: &[],
        media_types: &["application/vnd.nato.bindingdataobject+json"],
        signatures: &[],
        related_formats: &[],
    },
};
