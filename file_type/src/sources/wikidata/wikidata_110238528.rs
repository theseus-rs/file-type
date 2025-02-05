use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110238528: FileFormat = FileFormat {
    id: 110_238_528,
    source_type: SourceType::Wikidata,
    name: "Screenwriter 2000 Document",
    extensions: &["stw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
