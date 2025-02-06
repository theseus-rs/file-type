use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83868385: FileFormat = FileFormat {
    id: 83_868_385,
    source_type: SourceType::Wikidata,
    name: "SOSI, version 4.5",
    extensions: &["sos"],
    media_types: &["text/vnd.sosi"],
    signatures: &[],
    related_formats: &[],
};
