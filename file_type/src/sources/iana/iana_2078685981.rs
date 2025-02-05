use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2078685981: FileFormat = FileFormat {
    id: 2_078_685_981,
    source_type: SourceType::Iana,
    name: "vnd.intertrust.nncp",
    extensions: &[],
    media_types: &["application/vnd.intertrust.nncp"],
    signatures: &[],
    related_formats: &[],
};
