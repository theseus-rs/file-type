use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28692741: FileFormat = FileFormat {
    id: 28_692_741,
    source_type: SourceType::Wikidata,
    name: "FAV File Format",
    extensions: &["fav"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
