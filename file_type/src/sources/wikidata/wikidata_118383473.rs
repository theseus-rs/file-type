use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118383473: FileFormat = FileFormat {
    id: 118_383_473,
    source_type: SourceType::Wikidata,
    name: "Album Book file",
    extensions: &["opf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
