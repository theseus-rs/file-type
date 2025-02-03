use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1630469437: FileFormat = FileFormat {
    id: 1_630_469_437,
    source_type: SourceType::Iana,
    name: "vnd.software602.filler.form-xml-zip",
    extensions: &[],
    media_types: &["application/vnd.software602.filler.form-xml-zip"],
    internal_signatures: &[],
    related_formats: &[],
};
