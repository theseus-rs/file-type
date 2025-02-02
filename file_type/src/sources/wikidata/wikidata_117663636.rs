use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117663636: FileFormat = FileFormat {
    id: 117_663_636,
    source_type: SourceType::Wikidata,
    name: "The Print Shop Quick Prints File",
    extensions: &["php"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
