use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112661274: FileFormat = FileFormat {
    id: 112_661_274,
    source_type: SourceType::Wikidata,
    name: "Lightscape Solution file",
    extensions: &["ls"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
