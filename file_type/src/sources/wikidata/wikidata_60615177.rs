use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60615177: FileFormat = FileFormat {
    id: 60_615_177,
    source_type: SourceType::Wikidata,
    name: "Serif DrawPlus Drawing, version 5",
    extensions: &["dpp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
