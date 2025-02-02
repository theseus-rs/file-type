use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_62485589: FileFormat = FileFormat {
    id: 62_485_589,
    source_type: SourceType::Wikidata,
    name: "Navisworks Document, version 2012",
    extensions: &["nwc", "nwd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
