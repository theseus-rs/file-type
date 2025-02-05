use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113577536: FileFormat = FileFormat {
    id: 113_577_536,
    source_type: SourceType::Wikidata,
    name: "WinOnCD Image",
    extensions: &["c2d"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
