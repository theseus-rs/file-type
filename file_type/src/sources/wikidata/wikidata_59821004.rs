use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59821004: FileFormat = FileFormat {
    id: 59_821_004,
    source_type: SourceType::Wikidata,
    name: "Exchangeable Image File Format (Audio)",
    extensions: &["wav"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
