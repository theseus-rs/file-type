use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59616000: FileFormat = FileFormat {
    id: 59_616_000,
    source_type: SourceType::Wikidata,
    name: "Vectorworks file format, version 2010",
    extensions: &["vwx"],
    media_types: &["application/vnd.vectorworks"],
    internal_signatures: &[],
    related_formats: &[],
};
