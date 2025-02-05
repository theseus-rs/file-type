use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206838: FileFormat = FileFormat {
    id: 28_206_838,
    source_type: SourceType::Wikidata,
    name: "Palm bitmap",
    extensions: &["palm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
