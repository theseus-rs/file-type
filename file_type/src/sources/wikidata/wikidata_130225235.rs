use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130225235: FileFormat = FileFormat {
    id: 130_225_235,
    source_type: SourceType::Wikidata,
    name: "Limbo file format",
    extensions: &["b"],
    media_types: &["text/limbo"],
    internal_signatures: &[],
    related_formats: &[],
};
