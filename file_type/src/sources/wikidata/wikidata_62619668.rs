use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_62619668: FileFormat = FileFormat {
    id: 62_619_668,
    source_type: SourceType::Wikidata,
    name: "7-bit ANSI Text",
    extensions: &["ans"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
