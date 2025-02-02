use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_30102407: FileFormat = FileFormat {
    id: 30_102_407,
    source_type: SourceType::Wikidata,
    name: "Amateur Data Interchange Format, ADX variant, version 3.0.5",
    extensions: &["adx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
