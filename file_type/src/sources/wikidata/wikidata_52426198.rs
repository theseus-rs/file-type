use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52426198: FileFormat = FileFormat {
    id: 52_426_198,
    source_type: SourceType::Wikidata,
    name: "XYWrite for Windows Document, version 4",
    extensions: &["xy4", "xyw"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
