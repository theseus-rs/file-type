use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207152: FileFormat = FileFormat {
    id: 28_207_152,
    source_type: SourceType::Wikidata,
    name: "PTG",
    extensions: &["ptg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
