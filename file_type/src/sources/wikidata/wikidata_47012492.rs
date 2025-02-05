use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47012492: FileFormat = FileFormat {
    id: 47_012_492,
    source_type: SourceType::Wikidata,
    name: "Nota Bene Text file format",
    extensions: &["nb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
