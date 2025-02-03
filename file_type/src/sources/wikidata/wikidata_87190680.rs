use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_87190680: FileFormat = FileFormat {
    id: 87_190_680,
    source_type: SourceType::Wikidata,
    name: "X3D 3.1",
    extensions: &["x3d"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
