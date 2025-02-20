use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1735976981: FileType = FileType {
    file_format: &FileFormat {
        id: 1_735_976_981,
        source_type: SourceType::Iana,
        name: "vnd.nato.bindingdataobject+xml",
        extensions: &[],
        media_types: &["application/vnd.nato.bindingdataobject+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
