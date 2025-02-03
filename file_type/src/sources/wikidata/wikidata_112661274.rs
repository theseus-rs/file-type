use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112661274: FileFormat = FileFormat {
    id: 112_661_274,
    source_type: SourceType::Wikidata,
    name: "Lightscape Solution file",
    extensions: &["ls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
