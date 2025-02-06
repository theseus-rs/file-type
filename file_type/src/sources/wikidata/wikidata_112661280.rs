use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112661280: FileFormat = FileFormat {
    id: 112_661_280,
    source_type: SourceType::Wikidata,
    name: "Lightscape View file",
    extensions: &["vw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
