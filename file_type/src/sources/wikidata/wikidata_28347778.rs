use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28347778: FileFormat = FileFormat {
    id: 28_347_778,
    source_type: SourceType::Wikidata,
    name: "Zeno",
    extensions: &["zeno"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
