use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979408: FileFormat = FileFormat {
    id: 27_979_408,
    source_type: SourceType::Wikidata,
    name: "XNG",
    extensions: &["xng"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
