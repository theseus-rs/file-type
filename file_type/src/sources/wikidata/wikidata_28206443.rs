use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206443: FileFormat = FileFormat {
    id: 28_206_443,
    source_type: SourceType::Wikidata,
    name: "Kt Interchange File Format",
    extensions: &["kif", "kiff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
