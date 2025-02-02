use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_65533440: FileFormat = FileFormat {
    id: 65_533_440,
    source_type: SourceType::Wikidata,
    name: "BigOven Recipe Box File",
    extensions: &["crb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
