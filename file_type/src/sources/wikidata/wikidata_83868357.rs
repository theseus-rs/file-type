use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83868357: FileFormat = FileFormat {
    id: 83_868_357,
    source_type: SourceType::Wikidata,
    name: "SOSI, version 4",
    extensions: &["sos"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
