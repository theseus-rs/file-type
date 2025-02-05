use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60628185: FileFormat = FileFormat {
    id: 60_628_185,
    source_type: SourceType::Wikidata,
    name: "Wordperfect Secondary File, version 5",
    extensions: &["doc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
