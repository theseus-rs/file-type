use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1630469437: FileType = FileType {
    file_format: &FileFormat {
        id: 1_630_469_437,
        source_type: SourceType::Iana,
        name: "vnd.software602.filler.form-xml-zip",
        extensions: &[],
        media_types: &["application/vnd.software602.filler.form-xml-zip"],
        signatures: &[],
        related_formats: &[],
    },
};
