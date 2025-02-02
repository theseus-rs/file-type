use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110238528: FileFormat = FileFormat {
    id: 110_238_528,
    source_type: SourceType::Wikidata,
    name: "Screenwriter 2000 Document",
    extensions: &["stw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
