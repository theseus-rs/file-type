use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28757999: FileFormat = FileFormat {
    id: 28_757_999,
    source_type: SourceType::Wikidata,
    name: "Inform",
    extensions: &["i7"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
