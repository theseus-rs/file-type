use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60478880: FileFormat = FileFormat {
    id: 60_478_880,
    source_type: SourceType::Wikidata,
    name: "Serif DrawPlus Drawing, version 3",
    extensions: &["dpp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
