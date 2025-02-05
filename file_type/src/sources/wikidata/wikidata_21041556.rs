use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_21041556: FileFormat = FileFormat {
    id: 21_041_556,
    source_type: SourceType::Wikidata,
    name: "Music Editor format",
    extensions: &["med"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
