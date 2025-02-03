use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125847329: FileFormat = FileFormat {
    id: 125_847_329,
    source_type: SourceType::Wikidata,
    name: "D source code file",
    extensions: &["D"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
