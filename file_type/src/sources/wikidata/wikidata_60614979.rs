use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60614979: FileFormat = FileFormat {
    id: 60_614_979,
    source_type: SourceType::Wikidata,
    name: "Serif DrawPlus Drawing, version 4",
    extensions: &["dpp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
