use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62485305: FileFormat = FileFormat {
    id: 62_485_305,
    source_type: SourceType::Wikidata,
    name: "Navisworks Document, version 2010",
    extensions: &["nwc", "nwd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
