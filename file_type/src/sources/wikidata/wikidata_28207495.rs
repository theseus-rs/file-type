use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207495: FileFormat = FileFormat {
    id: 28_207_495,
    source_type: SourceType::Wikidata,
    name: "Wigmore Artist 64",
    extensions: &["a64", "wig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
