use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123594524: FileFormat = FileFormat {
    id: 123_594_524,
    source_type: SourceType::Wikidata,
    name: "TibetDoc Word Document",
    extensions: &["dct"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
