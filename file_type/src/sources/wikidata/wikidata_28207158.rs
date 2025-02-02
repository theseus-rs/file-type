use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207158: FileFormat = FileFormat {
    id: 28_207_158,
    source_type: SourceType::Wikidata,
    name: "Puzzle image",
    extensions: &["cm", "pzl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
