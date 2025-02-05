use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600469: FileFormat = FileFormat {
    id: 28_600_469,
    source_type: SourceType::Wikidata,
    name: "Distinguished Encoding Rules",
    extensions: &["der"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
