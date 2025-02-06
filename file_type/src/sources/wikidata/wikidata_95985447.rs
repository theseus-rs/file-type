use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_95985447: FileFormat = FileFormat {
    id: 95_985_447,
    source_type: SourceType::Wikidata,
    name: "Affymetrix GIN file format",
    extensions: &["gin"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
