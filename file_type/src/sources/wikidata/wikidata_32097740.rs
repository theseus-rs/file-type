use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_32097740: FileFormat = FileFormat {
    id: 32_097_740,
    source_type: SourceType::Wikidata,
    name: "DAT",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
