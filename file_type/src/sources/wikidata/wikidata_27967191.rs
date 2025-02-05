use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967191: FileFormat = FileFormat {
    id: 27_967_191,
    source_type: SourceType::Wikidata,
    name: "Grave Composer module",
    extensions: &["wow"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
