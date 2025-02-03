use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979516: FileFormat = FileFormat {
    id: 27_979_516,
    source_type: SourceType::Wikidata,
    name: "Manga Studio Page",
    extensions: &["cpg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
