use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127700023: FileFormat = FileFormat {
    id: 127_700_023,
    source_type: SourceType::Wikidata,
    name: "Gravity file",
    extensions: &["grv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
