use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_83868394: FileFormat = FileFormat {
    id: 83_868_394,
    source_type: SourceType::Wikidata,
    name: "SOSI, version 8.1",
    extensions: &["sos"],
    media_types: &["text/vnd.sosi"],
    internal_signatures: &[],
    related_formats: &[],
};
