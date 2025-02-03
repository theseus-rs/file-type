use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118492392: FileFormat = FileFormat {
    id: 118_492_392,
    source_type: SourceType::Wikidata,
    name: "Quicken 3 Database File",
    extensions: &["qst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
