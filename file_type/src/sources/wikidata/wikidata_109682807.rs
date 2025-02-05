use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109682807: FileFormat = FileFormat {
    id: 109_682_807,
    source_type: SourceType::Wikidata,
    name: "Sinar Digital Back format",
    extensions: &["sti"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
