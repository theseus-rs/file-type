use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867402: FileFormat = FileFormat {
    id: 105_867_402,
    source_type: SourceType::Wikidata,
    name: "NSIS script (with rem)",
    extensions: &["nsi"],
    media_types: &["text/plain", "text/x-nsis"],
    internal_signatures: &[],
    related_formats: &[],
};
