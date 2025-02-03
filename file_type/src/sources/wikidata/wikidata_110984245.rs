use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110984245: FileFormat = FileFormat {
    id: 110_984_245,
    source_type: SourceType::Wikidata,
    name: "Video Paint File",
    extensions: &["uvp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
