use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206066: FileFormat = FileFormat {
    id: 28_206_066,
    source_type: SourceType::Wikidata,
    name: "View ST/TT TT-Low",
    extensions: &["PI4"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
