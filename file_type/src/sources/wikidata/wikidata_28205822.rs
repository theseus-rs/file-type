use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205822: FileFormat = FileFormat {
    id: 28_205_822,
    source_type: SourceType::Wikidata,
    name: "CD5",
    extensions: &["cd5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
