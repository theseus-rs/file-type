use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114132929: FileFormat = FileFormat {
    id: 114_132_929,
    source_type: SourceType::Wikidata,
    name: "Constraint File Format",
    extensions: &["con"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
