use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_21040799: FileFormat = FileFormat {
    id: 21_040_799,
    source_type: SourceType::Wikidata,
    name: "MadTracker 2 format",
    extensions: &["mt2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
