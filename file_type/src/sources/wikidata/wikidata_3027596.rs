use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3027596: FileFormat = FileFormat {
    id: 3_027_596,
    source_type: SourceType::Wikidata,
    name: "DGN",
    extensions: &["dgn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
