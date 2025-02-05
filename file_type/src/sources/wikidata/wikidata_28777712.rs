use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28777712: FileFormat = FileFormat {
    id: 28_777_712,
    source_type: SourceType::Wikidata,
    name: "NFF",
    extensions: &["nff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
