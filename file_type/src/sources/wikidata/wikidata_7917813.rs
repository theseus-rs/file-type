use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7917813: FileFormat = FileFormat {
    id: 7_917_813,
    source_type: SourceType::Wikidata,
    name: "Vector Product Format",
    extensions: &["vpf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
