use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122074126: FileFormat = FileFormat {
    id: 122_074_126,
    source_type: SourceType::Wikidata,
    name: "Score file",
    extensions: &["pge"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
