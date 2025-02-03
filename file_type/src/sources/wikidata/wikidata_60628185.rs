use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60628185: FileFormat = FileFormat {
    id: 60_628_185,
    source_type: SourceType::Wikidata,
    name: "Wordperfect Secondary File, version 5",
    extensions: &["doc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
