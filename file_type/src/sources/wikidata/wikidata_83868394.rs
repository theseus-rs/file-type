use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83868394: FileFormat = FileFormat {
    id: 83_868_394,
    source_type: SourceType::Wikidata,
    name: "SOSI, version 8.1",
    extensions: &["sos"],
    media_types: &["text/vnd.sosi"],
    signatures: &[],
    related_formats: &[],
};
