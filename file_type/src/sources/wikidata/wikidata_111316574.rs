use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111316574: FileFormat = FileFormat {
    id: 111_316_574,
    source_type: SourceType::Wikidata,
    name: "Sample Cell II PC instrument",
    extensions: &["ins"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
