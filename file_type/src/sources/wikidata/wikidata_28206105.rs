use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206105: FileFormat = FileFormat {
    id: 28_206_105,
    source_type: SourceType::Wikidata,
    name: "Falcon True Color",
    extensions: &["ftc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
