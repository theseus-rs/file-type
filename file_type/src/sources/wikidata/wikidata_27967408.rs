use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967408: FileFormat = FileFormat {
    id: 27_967_408,
    source_type: SourceType::Wikidata,
    name: "Codisk Audio File",
    extensions: &["caf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
