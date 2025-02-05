use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856185: FileFormat = FileFormat {
    id: 105_856_185,
    source_type: SourceType::Wikidata,
    name: "MPlayer font Description",
    extensions: &["desc"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
