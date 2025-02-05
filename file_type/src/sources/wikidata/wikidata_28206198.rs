use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206198: FileFormat = FileFormat {
    id: 28_206_198,
    source_type: SourceType::Wikidata,
    name: "GodPaint",
    extensions: &["god"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
