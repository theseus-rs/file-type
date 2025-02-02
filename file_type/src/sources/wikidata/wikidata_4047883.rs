use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_4047883: FileFormat = FileFormat {
    id: 4_047_883,
    source_type: SourceType::Wikidata,
    name: "long-term prediction",
    extensions: &["gsm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
