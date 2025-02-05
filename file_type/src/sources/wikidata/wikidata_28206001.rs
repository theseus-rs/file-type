use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206001: FileFormat = FileFormat {
    id: 28_206_001,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive Device-dependent Data",
    extensions: &["i8"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
