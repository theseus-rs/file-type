use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118721205: FileFormat = FileFormat {
    id: 118_721_205,
    source_type: SourceType::Wikidata,
    name: "PZZ File",
    extensions: &["pzz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
