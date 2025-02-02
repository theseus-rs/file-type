use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48805492: FileFormat = FileFormat {
    id: 48_805_492,
    source_type: SourceType::Wikidata,
    name: "ChiWriter Document",
    extensions: &["chi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
