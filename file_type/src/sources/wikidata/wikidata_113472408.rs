use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113472408: FileFormat = FileFormat {
    id: 113_472_408,
    source_type: SourceType::Wikidata,
    name: "Glyphs Character Data",
    extensions: &["glyphs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
