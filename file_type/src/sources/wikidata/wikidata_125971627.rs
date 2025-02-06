use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125971627: FileFormat = FileFormat {
    id: 125_971_627,
    source_type: SourceType::Wikidata,
    name: "gemtext",
    extensions: &["gmi"],
    media_types: &["text/gemini"],
    signatures: &[],
    related_formats: &[],
};
