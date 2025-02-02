use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2145498: FileFormat = FileFormat {
    id: 2_145_498,
    source_type: SourceType::Wikidata,
    name: "Requirements Interchange Format",
    extensions: &["reqif", "reqifz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
