use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979513: FileFormat = FileFormat {
    id: 27_979_513,
    source_type: SourceType::Wikidata,
    name: "Manga Studio Story",
    extensions: &["cst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
