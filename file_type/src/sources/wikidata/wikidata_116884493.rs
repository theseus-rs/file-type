use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116884493: FileFormat = FileFormat {
    id: 116_884_493,
    source_type: SourceType::Wikidata,
    name: "EPS Tiff Preview",
    extensions: &["eps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
