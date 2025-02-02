use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125971627: FileFormat = FileFormat {
    id: 125_971_627,
    source_type: SourceType::Wikidata,
    name: "gemtext",
    extensions: &["gmi"],
    media_types: &["text/gemini"],
    internal_signatures: &[],
    related_formats: &[],
};
