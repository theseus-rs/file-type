use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66310986: FileFormat = FileFormat {
    id: 66_310_986,
    source_type: SourceType::Wikidata,
    name: "Shopping List file format",
    extensions: &["sl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
