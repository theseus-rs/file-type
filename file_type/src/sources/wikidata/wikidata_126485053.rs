use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126485053: FileFormat = FileFormat {
    id: 126_485_053,
    source_type: SourceType::Wikidata,
    name: "Omnis Sudio Project Library file",
    extensions: &["lbs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
