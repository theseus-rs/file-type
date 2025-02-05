use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111511710: FileFormat = FileFormat {
    id: 111_511_710,
    source_type: SourceType::Wikidata,
    name: "TGIF File Format",
    extensions: &["obj", "tgif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
