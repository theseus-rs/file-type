use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_65533770: FileFormat = FileFormat {
    id: 65_533_770,
    source_type: SourceType::Wikidata,
    name: "Open Recipe Format",
    extensions: &["orf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
