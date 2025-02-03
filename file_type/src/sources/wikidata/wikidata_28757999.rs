use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757999: FileFormat = FileFormat {
    id: 28_757_999,
    source_type: SourceType::Wikidata,
    name: "Inform",
    extensions: &["i7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
