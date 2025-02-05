use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126828176: FileFormat = FileFormat {
    id: 126_828_176,
    source_type: SourceType::Wikidata,
    name: "Forth source code file",
    extensions: &["fs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
