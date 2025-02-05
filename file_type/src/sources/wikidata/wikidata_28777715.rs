use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28777715: FileFormat = FileFormat {
    id: 28_777_715,
    source_type: SourceType::Wikidata,
    name: "NSIS",
    extensions: &["exe"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
