use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207252: FileFormat = FileFormat {
    id: 28_207_252,
    source_type: SourceType::Wikidata,
    name: "SCR",
    extensions: &["scr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
