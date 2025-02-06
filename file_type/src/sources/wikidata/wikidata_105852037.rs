use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852037: FileFormat = FileFormat {
    id: 105_852_037,
    source_type: SourceType::Wikidata,
    name: "Digital Micrograph Script",
    extensions: &["s"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
