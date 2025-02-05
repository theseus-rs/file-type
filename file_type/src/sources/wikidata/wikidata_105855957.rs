use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855957: FileFormat = FileFormat {
    id: 105_855_957,
    source_type: SourceType::Wikidata,
    name: "ColdFusion Verity engine fields definition",
    extensions: &["ddd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
