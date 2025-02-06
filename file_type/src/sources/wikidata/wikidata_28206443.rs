use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206443: FileFormat = FileFormat {
    id: 28_206_443,
    source_type: SourceType::Wikidata,
    name: "Kt Interchange File Format",
    extensions: &["kif", "kiff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
