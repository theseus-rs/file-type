use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27960787: FileFormat = FileFormat {
    id: 27_960_787,
    source_type: SourceType::Wikidata,
    name: "HS2",
    extensions: &["hs2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
