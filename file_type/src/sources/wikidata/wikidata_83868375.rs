use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83868375: FileFormat = FileFormat {
    id: 83_868_375,
    source_type: SourceType::Wikidata,
    name: "SOSI, version 4.1",
    extensions: &["sos"],
    media_types: &["text/vnd.sosi"],
    signatures: &[],
    related_formats: &[],
};
