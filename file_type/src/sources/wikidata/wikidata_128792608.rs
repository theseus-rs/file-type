use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128792608: FileFormat = FileFormat {
    id: 128_792_608,
    source_type: SourceType::Wikidata,
    name: "DAX formula file",
    extensions: &["dax"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
