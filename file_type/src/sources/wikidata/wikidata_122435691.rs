use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122435691: FileFormat = FileFormat {
    id: 122_435_691,
    source_type: SourceType::Wikidata,
    name: "NovaBACKUP Job",
    extensions: &["nbk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
