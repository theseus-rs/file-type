use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122435691: FileFormat = FileFormat {
    id: 122_435_691,
    source_type: SourceType::Wikidata,
    name: "NovaBACKUP Job",
    extensions: &["nbk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
