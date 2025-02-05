use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207374: FileFormat = FileFormat {
    id: 28_207_374,
    source_type: SourceType::Wikidata,
    name: "Technicolor Dream COL",
    extensions: &["col"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
