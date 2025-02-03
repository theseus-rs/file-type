use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113577536: FileFormat = FileFormat {
    id: 113_577_536,
    source_type: SourceType::Wikidata,
    name: "WinOnCD Image",
    extensions: &["c2d"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
