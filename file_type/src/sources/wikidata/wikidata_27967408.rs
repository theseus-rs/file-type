use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967408: FileFormat = FileFormat {
    id: 27_967_408,
    source_type: SourceType::Wikidata,
    name: "Codisk Audio File",
    extensions: &["caf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
