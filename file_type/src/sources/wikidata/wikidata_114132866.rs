use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114132866: FileFormat = FileFormat {
    id: 114_132_866,
    source_type: SourceType::Wikidata,
    name: "Connectivity Table file format",
    extensions: &["ct"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
