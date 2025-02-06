use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111341736: FileFormat = FileFormat {
    id: 111_341_736,
    source_type: SourceType::Wikidata,
    name: "Sound Designer II flattened file",
    extensions: &["sd2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
