use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967128: FileFormat = FileFormat {
    id: 27_967_128,
    source_type: SourceType::Wikidata,
    name: "DMC",
    extensions: &["dmc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
