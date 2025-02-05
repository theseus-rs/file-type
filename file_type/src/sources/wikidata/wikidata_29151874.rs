use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29151874: FileFormat = FileFormat {
    id: 29_151_874,
    source_type: SourceType::Wikidata,
    name: "QRT Ray Tracer scene description",
    extensions: &["qrt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
