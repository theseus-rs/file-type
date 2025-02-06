use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206565: FileFormat = FileFormat {
    id: 28_206_565,
    source_type: SourceType::Wikidata,
    name: "MicroDesign Area",
    extensions: &["mda"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
