use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118492392: FileFormat = FileFormat {
    id: 118_492_392,
    puid: "wikidata/118492392",
    name: "Quicken 3 Database File",
    extensions: &["qst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
