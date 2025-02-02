use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600760: FileFormat = FileFormat {
    id: 28_600_760,
    source_type: SourceType::Wikidata,
    name: "Early Mind Manager XML format",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
