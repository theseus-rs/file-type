use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205801: FileFormat = FileFormat {
    id: 28_205_801,
    source_type: SourceType::Wikidata,
    name: "IMG Picture Format",
    extensions: &["img"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
