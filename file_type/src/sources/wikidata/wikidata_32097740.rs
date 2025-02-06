use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_32097740: FileFormat = FileFormat {
    id: 32_097_740,
    source_type: SourceType::Wikidata,
    name: "DAT",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
