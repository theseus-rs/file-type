use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34307024: FileFormat = FileFormat {
    id: 34_307_024,
    source_type: SourceType::Wikidata,
    name: "Scratch Project Sprite",
    extensions: &["sprite"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
