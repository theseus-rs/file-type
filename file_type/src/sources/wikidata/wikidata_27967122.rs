use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967122: FileFormat = FileFormat {
    id: 27_967_122,
    source_type: SourceType::Wikidata,
    name: "Brian Postma SoundMon v2.x & v3.x module",
    extensions: &["bp3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
