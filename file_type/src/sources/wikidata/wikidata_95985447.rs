use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_95985447: FileFormat = FileFormat {
    id: 95_985_447,
    source_type: SourceType::Wikidata,
    name: "Affymetrix GIN file format",
    extensions: &["gin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
