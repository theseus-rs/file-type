use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207114: FileFormat = FileFormat {
    id: 28_207_114,
    source_type: SourceType::Wikidata,
    name: "The New Print Shop Names file",
    extensions: &["pnm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
