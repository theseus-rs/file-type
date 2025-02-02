use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_6717333: FileFormat = FileFormat {
    id: 6_717_333,
    source_type: SourceType::Wikidata,
    name: "Mathematical Programming System",
    extensions: &["mps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
