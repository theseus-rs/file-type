use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47896997: FileFormat = FileFormat {
    id: 47_896_997,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange Format Style Extract",
    extensions: &["dxx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
