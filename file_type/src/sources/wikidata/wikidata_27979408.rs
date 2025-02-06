use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979408: FileFormat = FileFormat {
    id: 27_979_408,
    source_type: SourceType::Wikidata,
    name: "XNG",
    extensions: &["xng"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
