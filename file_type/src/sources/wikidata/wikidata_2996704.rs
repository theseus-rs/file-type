use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2996704: FileFormat = FileFormat {
    id: 2_996_704,
    source_type: SourceType::Wikidata,
    name: ".htpasswd",
    extensions: &["htpasswd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
