use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116446363: FileFormat = FileFormat {
    id: 116_446_363,
    source_type: SourceType::Wikidata,
    name: "Work File",
    extensions: &["w"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
