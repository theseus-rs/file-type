use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100323885: FileFormat = FileFormat {
    id: 100_323_885,
    source_type: SourceType::Wikidata,
    name: "Corel Gallery Clipart",
    extensions: &["bmf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
