use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28345059: FileFormat = FileFormat {
    id: 28_345_059,
    source_type: SourceType::Wikidata,
    name: "XP3",
    extensions: &["xp3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
