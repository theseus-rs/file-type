use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116884493: FileFormat = FileFormat {
    id: 116_884_493,
    source_type: SourceType::Wikidata,
    name: "EPS Tiff Preview",
    extensions: &["eps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
