use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855534: FileFormat = FileFormat {
    id: 105_855_534,
    source_type: SourceType::Wikidata,
    name: "Open Digital Rights Language",
    extensions: &["dr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
