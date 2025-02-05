use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207158: FileFormat = FileFormat {
    id: 28_207_158,
    source_type: SourceType::Wikidata,
    name: "Puzzle image",
    extensions: &["cm", "pzl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
