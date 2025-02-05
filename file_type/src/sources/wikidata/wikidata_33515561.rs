use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_33515561: FileFormat = FileFormat {
    id: 33_515_561,
    source_type: SourceType::Wikidata,
    name: "LAS 1.3 file format",
    extensions: &["las", "laz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
