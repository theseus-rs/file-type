use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_72724699: FileFormat = FileFormat {
    id: 72_724_699,
    source_type: SourceType::Wikidata,
    name: "Newsletters And More document",
    extensions: &["nam"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
