use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131471298: FileFormat = FileFormat {
    id: 131_471_298,
    source_type: SourceType::Wikidata,
    name: "MGH file format",
    extensions: &["mgh"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
