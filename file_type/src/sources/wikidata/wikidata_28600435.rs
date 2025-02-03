use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600435: FileFormat = FileFormat {
    id: 28_600_435,
    source_type: SourceType::Wikidata,
    name: "Consolidated.db",
    extensions: &["db"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
