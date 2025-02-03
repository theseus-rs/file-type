use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_10397009: FileFormat = FileFormat {
    id: 10_397_009,
    source_type: SourceType::Wikidata,
    name: "Arachne Plugin Manager file format",
    extensions: &["apm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
