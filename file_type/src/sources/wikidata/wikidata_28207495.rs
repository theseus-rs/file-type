use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207495: FileFormat = FileFormat {
    id: 28_207_495,
    source_type: SourceType::Wikidata,
    name: "Wigmore Artist 64",
    extensions: &["a64", "wig"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
