use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207374: FileFormat = FileFormat {
    id: 28_207_374,
    source_type: SourceType::Wikidata,
    name: "Technicolor Dream COL",
    extensions: &["col"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
