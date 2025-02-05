use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27966915: FileFormat = FileFormat {
    id: 27_966_915,
    source_type: SourceType::Wikidata,
    name: "NES Sound Format Extended",
    extensions: &["nsfe"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
