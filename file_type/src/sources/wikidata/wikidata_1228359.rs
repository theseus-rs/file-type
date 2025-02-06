use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1228359: FileFormat = FileFormat {
    id: 1_228_359,
    source_type: SourceType::Wikidata,
    name: "Disc Description Protocol",
    extensions: &["ddp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
