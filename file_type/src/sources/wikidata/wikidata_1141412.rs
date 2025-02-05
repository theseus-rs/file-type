use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1141412: FileFormat = FileFormat {
    id: 1_141_412,
    source_type: SourceType::Wikidata,
    name: "INI file",
    extensions: &["ini"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
