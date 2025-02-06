use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50604550: FileFormat = FileFormat {
    id: 50_604_550,
    source_type: SourceType::Wikidata,
    name: "FileMaker Pro Database, version 2",
    extensions: &["fm"],
    media_types: &["application/x-filemaker"],
    signatures: &[],
    related_formats: &[],
};
