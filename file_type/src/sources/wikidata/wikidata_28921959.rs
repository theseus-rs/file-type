use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28921959: FileFormat = FileFormat {
    id: 28_921_959,
    source_type: SourceType::Wikidata,
    name: "Keyhole Markup Language Zipped",
    extensions: &["kmz"],
    media_types: &["application/vnd.google-earth.kmz"],
    internal_signatures: &[],
    related_formats: &[],
};
