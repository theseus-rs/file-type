use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28052835: FileFormat = FileFormat {
    id: 28_052_835,
    source_type: SourceType::Wikidata,
    name: "Digital Replica Plus",
    extensions: &["epub"],
    media_types: &["application/epub+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
