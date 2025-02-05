use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2996704: FileFormat = FileFormat {
    id: 2_996_704,
    source_type: SourceType::Wikidata,
    name: ".htpasswd",
    extensions: &["htpasswd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
