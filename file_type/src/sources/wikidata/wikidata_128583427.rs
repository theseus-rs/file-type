use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128583427: FileFormat = FileFormat {
    id: 128_583_427,
    source_type: SourceType::Wikidata,
    name: "ABAP file format",
    extensions: &["abap"],
    media_types: &["text/x-abap"],
    internal_signatures: &[],
    related_formats: &[],
};
