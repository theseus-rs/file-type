use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_52425808: FileFormat = FileFormat {
    id: 52_425_808,
    source_type: SourceType::Wikidata,
    name: "Vista Pro Graphics",
    extensions: &["dem"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
