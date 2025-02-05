use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650304: FileFormat = FileFormat {
    id: 29_650_304,
    source_type: SourceType::Wikidata,
    name: "PRT scene description",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
