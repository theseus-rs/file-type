use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206198: FileFormat = FileFormat {
    id: 28_206_198,
    source_type: SourceType::Wikidata,
    name: "GodPaint",
    extensions: &["god"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
