use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34303434: FileFormat = FileFormat {
    id: 34_303_434,
    source_type: SourceType::Wikidata,
    name: "SYSDOOM script",
    extensions: &["doom"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
