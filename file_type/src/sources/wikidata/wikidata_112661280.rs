use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112661280: FileFormat = FileFormat {
    id: 112_661_280,
    source_type: SourceType::Wikidata,
    name: "Lightscape View file",
    extensions: &["vw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
