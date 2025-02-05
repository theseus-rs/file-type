use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867323: FileFormat = FileFormat {
    id: 105_867_323,
    source_type: SourceType::Wikidata,
    name: "OS/2 Network Information File (with rem)",
    extensions: &["nif"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
