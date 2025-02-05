use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72724699: FileFormat = FileFormat {
    id: 72_724_699,
    source_type: SourceType::Wikidata,
    name: "Newsletters And More document",
    extensions: &["nam"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
