use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600435: FileFormat = FileFormat {
    id: 28_600_435,
    source_type: SourceType::Wikidata,
    name: "Consolidated.db",
    extensions: &["db"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
