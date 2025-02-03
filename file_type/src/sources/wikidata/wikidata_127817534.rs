use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127817534: FileFormat = FileFormat {
    id: 127_817_534,
    source_type: SourceType::Wikidata,
    name: "gp script",
    extensions: &["gp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
