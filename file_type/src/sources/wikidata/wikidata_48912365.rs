use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48912365: FileFormat = FileFormat {
    id: 48_912_365,
    source_type: SourceType::Wikidata,
    name: "InterBase Database",
    extensions: &["gdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
