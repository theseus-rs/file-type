use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28777712: FileFormat = FileFormat {
    id: 28_777_712,
    source_type: SourceType::Wikidata,
    name: "NFF",
    extensions: &["nff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
