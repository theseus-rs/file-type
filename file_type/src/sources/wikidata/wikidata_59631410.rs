use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59631410: FileFormat = FileFormat {
    id: 59_631_410,
    source_type: SourceType::Wikidata,
    name: "Navisworks Document",
    extensions: &["nwc", "nwd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
