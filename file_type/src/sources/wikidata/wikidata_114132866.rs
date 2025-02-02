use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114132866: FileFormat = FileFormat {
    id: 114_132_866,
    source_type: SourceType::Wikidata,
    name: "Connectivity Table file format",
    extensions: &["ct"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
