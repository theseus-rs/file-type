use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60478880: FileFormat = FileFormat {
    id: 60_478_880,
    source_type: SourceType::Wikidata,
    name: "Serif DrawPlus Drawing, version 3",
    extensions: &["dpp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
