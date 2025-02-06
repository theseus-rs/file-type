use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27960082: FileFormat = FileFormat {
    id: 27_960_082,
    source_type: SourceType::Wikidata,
    name: "DCT",
    extensions: &["dct", "wav"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
