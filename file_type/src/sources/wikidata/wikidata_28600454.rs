use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600454: FileFormat = FileFormat {
    id: 28_600_454,
    source_type: SourceType::Wikidata,
    name: "DB (Watcom-SQL)",
    extensions: &["db"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
