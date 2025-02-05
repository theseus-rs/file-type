use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867402: FileFormat = FileFormat {
    id: 105_867_402,
    source_type: SourceType::Wikidata,
    name: "NSIS script (with rem)",
    extensions: &["nsi"],
    media_types: &["text/plain", "text/x-nsis"],
    signatures: &[],
    related_formats: &[],
};
