use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47012492: FileFormat = FileFormat {
    id: 47_012_492,
    source_type: SourceType::Wikidata,
    name: "Nota Bene Text file format",
    extensions: &["nb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
