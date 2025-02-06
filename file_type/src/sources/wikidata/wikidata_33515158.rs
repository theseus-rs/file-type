use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_33515158: FileFormat = FileFormat {
    id: 33_515_158,
    source_type: SourceType::Wikidata,
    name: "LAS 1.1",
    extensions: &["las", "laz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
