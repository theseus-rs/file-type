use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28758212: FileFormat = FileFormat {
    id: 28_758_212,
    source_type: SourceType::Wikidata,
    name: "Street Atlas USA Draw File",
    extensions: &["an1"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
