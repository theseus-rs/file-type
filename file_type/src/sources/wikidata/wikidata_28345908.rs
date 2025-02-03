use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28345908: FileFormat = FileFormat {
    id: 28_345_908,
    source_type: SourceType::Wikidata,
    name: "Apple Preferred",
    extensions: &["apf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
