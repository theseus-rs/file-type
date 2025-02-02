use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856185: FileFormat = FileFormat {
    id: 105_856_185,
    source_type: SourceType::Wikidata,
    name: "MPlayer font Description",
    extensions: &["desc"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
