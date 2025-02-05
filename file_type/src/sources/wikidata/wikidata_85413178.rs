use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_85413178: FileFormat = FileFormat {
    id: 85_413_178,
    source_type: SourceType::Wikidata,
    name: "PTGui Project File 10",
    extensions: &["pts"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
