use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111262844: FileFormat = FileFormat {
    id: 111_262_844,
    source_type: SourceType::Wikidata,
    name: "AKAI S5000/S6000 program",
    extensions: &["akai"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
