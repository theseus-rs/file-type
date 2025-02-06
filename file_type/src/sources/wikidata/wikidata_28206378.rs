use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206378: FileFormat = FileFormat {
    id: 28_206_378,
    source_type: SourceType::Wikidata,
    name: "IPI",
    extensions: &["ipi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
