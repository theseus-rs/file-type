use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124845089: FileFormat = FileFormat {
    id: 124_845_089,
    source_type: SourceType::Wikidata,
    name: "mh",
    extensions: &["mh"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
