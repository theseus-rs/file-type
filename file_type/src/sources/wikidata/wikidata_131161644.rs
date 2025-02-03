use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131161644: FileFormat = FileFormat {
    id: 131_161_644,
    source_type: SourceType::Wikidata,
    name: "SRCINFO file format",
    extensions: &["SRCINFO"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
