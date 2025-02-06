use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27996230: FileFormat = FileFormat {
    id: 27_996_230,
    source_type: SourceType::Wikidata,
    name: "FileMaker Pro Database, version 5",
    extensions: &["fp5"],
    media_types: &["application/x-filemaker"],
    signatures: &[],
    related_formats: &[],
};
