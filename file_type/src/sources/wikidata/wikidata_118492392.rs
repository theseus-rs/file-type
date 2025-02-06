use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118492392: FileFormat = FileFormat {
    id: 118_492_392,
    source_type: SourceType::Wikidata,
    name: "Quicken 3 Database File",
    extensions: &["qst"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
