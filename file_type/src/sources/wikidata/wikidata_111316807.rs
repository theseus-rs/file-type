use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111316807: FileFormat = FileFormat {
    id: 111_316_807,
    source_type: SourceType::Wikidata,
    name: "Kurzweil K2500 file",
    extensions: &["k25"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
