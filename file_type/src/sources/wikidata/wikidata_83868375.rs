use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_83868375: FileFormat = FileFormat {
    id: 83_868_375,
    source_type: SourceType::Wikidata,
    name: "SOSI, version 4.1",
    extensions: &["sos"],
    media_types: &["text/vnd.sosi"],
    internal_signatures: &[],
    related_formats: &[],
};
