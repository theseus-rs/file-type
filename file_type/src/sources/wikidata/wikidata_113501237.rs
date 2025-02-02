use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113501237: FileFormat = FileFormat {
    id: 113_501_237,
    source_type: SourceType::Wikidata,
    name: "Time Stamp Token",
    extensions: &["tst"],
    media_types: &["application/vnd.etsi.timestamp-token"],
    internal_signatures: &[],
    related_formats: &[],
};
