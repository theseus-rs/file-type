use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66310986: FileFormat = FileFormat {
    id: 66_310_986,
    source_type: SourceType::Wikidata,
    name: "Shopping List file format",
    extensions: &["sl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
