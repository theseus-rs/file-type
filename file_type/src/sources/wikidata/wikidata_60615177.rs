use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60615177: FileFormat = FileFormat {
    id: 60_615_177,
    source_type: SourceType::Wikidata,
    name: "Serif DrawPlus Drawing, version 5",
    extensions: &["dpp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
