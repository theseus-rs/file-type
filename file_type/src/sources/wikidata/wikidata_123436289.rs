use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123436289: FileFormat = FileFormat {
    id: 123_436_289,
    source_type: SourceType::Wikidata,
    name: "DARC-F1 file",
    extensions: &["f1d"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
