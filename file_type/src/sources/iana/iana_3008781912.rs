use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3008781912: FileFormat = FileFormat {
    id: 3_008_781_912,
    source_type: SourceType::Iana,
    name: "vnd.oasis.opendocument.formula-template",
    extensions: &[],
    media_types: &["application/vnd.oasis.opendocument.formula-template"],
    signatures: &[],
    related_formats: &[],
};
