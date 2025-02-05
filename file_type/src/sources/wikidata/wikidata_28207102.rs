use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207102: FileFormat = FileFormat {
    id: 28_207_102,
    source_type: SourceType::Wikidata,
    name: "The Print Shop Names file",
    extensions: &["nam"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
