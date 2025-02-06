use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600454: FileFormat = FileFormat {
    id: 28_600_454,
    source_type: SourceType::Wikidata,
    name: "DB (Watcom-SQL)",
    extensions: &["db"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
