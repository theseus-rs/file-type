use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206218: FileFormat = FileFormat {
    id: 28_206_218,
    source_type: SourceType::Wikidata,
    name: "GRF",
    extensions: &["grf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
