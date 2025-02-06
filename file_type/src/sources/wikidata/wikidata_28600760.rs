use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600760: FileFormat = FileFormat {
    id: 28_600_760,
    source_type: SourceType::Wikidata,
    name: "Early Mind Manager XML format",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
