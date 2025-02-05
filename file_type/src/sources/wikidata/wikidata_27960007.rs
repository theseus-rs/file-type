use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27960007: FileFormat = FileFormat {
    id: 27_960_007,
    source_type: SourceType::Wikidata,
    name: "RK Audio",
    extensions: &["rka"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
