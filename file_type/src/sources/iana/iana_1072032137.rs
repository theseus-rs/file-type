use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1072032137: FileFormat = FileFormat {
    id: 1_072_032_137,
    source_type: SourceType::Iana,
    name: "vnd.motorola.flexsuite.adsi",
    extensions: &[],
    media_types: &["application/vnd.motorola.flexsuite.adsi"],
    internal_signatures: &[],
    related_formats: &[],
};
