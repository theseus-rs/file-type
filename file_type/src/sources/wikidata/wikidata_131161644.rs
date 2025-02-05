use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131161644: FileFormat = FileFormat {
    id: 131_161_644,
    source_type: SourceType::Wikidata,
    name: "SRCINFO file format",
    extensions: &["SRCINFO"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
