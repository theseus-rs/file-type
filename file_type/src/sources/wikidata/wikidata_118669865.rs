use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118669865: FileFormat = FileFormat {
    id: 118_669_865,
    source_type: SourceType::Wikidata,
    name: "Manga Studio 3D Dialog file",
    extensions: &["csd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
