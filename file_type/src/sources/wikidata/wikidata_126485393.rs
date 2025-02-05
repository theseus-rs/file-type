use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126485393: FileFormat = FileFormat {
    id: 126_485_393,
    source_type: SourceType::Wikidata,
    name: "Comic Book ACE Archive",
    extensions: &["cba"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
