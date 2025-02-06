use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126085944: FileFormat = FileFormat {
    id: 126_085_944,
    source_type: SourceType::Wikidata,
    name: "IMF Package Asset Map",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
