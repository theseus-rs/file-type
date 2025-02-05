use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124097900: FileFormat = FileFormat {
    id: 124_097_900,
    source_type: SourceType::Wikidata,
    name: ".txt file",
    extensions: &["txt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
