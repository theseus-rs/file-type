use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127700023: FileFormat = FileFormat {
    id: 127_700_023,
    source_type: SourceType::Wikidata,
    name: "Gravity file",
    extensions: &["grv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
