use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206085: FileFormat = FileFormat {
    id: 28_206_085,
    source_type: SourceType::Wikidata,
    name: "TT High Resolution",
    extensions: &["PI7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
