use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27960146: FileFormat = FileFormat {
    id: 27_960_146,
    source_type: SourceType::Wikidata,
    name: "X2A",
    extensions: &["x2a"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
