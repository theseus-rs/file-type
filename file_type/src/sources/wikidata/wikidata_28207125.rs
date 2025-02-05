use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207125: FileFormat = FileFormat {
    id: 28_207_125,
    source_type: SourceType::Wikidata,
    name: "The New Print Shop Graphics file",
    extensions: &["pog"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
