use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58103380: FileFormat = FileFormat {
    id: 58_103_380,
    source_type: SourceType::Wikidata,
    name: "eRuby HTML document",
    extensions: &["rhtm", "rhtml"],
    media_types: &["text/html+ruby"],
    signatures: &[],
    related_formats: &[],
};
