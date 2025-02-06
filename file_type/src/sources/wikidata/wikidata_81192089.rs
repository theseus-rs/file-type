use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_81192089: FileFormat = FileFormat {
    id: 81_192_089,
    source_type: SourceType::Wikidata,
    name: "Infinity Engine Compiled Script",
    extensions: &["bcs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
