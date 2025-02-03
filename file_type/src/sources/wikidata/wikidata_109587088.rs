use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109587088: FileFormat = FileFormat {
    id: 109_587_088,
    source_type: SourceType::Wikidata,
    name: "EasyPhoto Gallery",
    extensions: &["gal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
