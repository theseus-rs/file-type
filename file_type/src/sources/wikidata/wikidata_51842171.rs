use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51842171: FileFormat = FileFormat {
    id: 51_842_171,
    source_type: SourceType::Wikidata,
    name: "MacPaint Graphics",
    extensions: &["pnt"],
    media_types: &["image/mac"],
    signatures: &[],
    related_formats: &[],
};
