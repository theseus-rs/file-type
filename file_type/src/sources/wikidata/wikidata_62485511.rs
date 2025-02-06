use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62485511: FileFormat = FileFormat {
    id: 62_485_511,
    source_type: SourceType::Wikidata,
    name: "Navisworks Document, version 2011",
    extensions: &["nwc", "nwd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
