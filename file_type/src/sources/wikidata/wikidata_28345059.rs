use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28345059: FileFormat = FileFormat {
    id: 28_345_059,
    source_type: SourceType::Wikidata,
    name: "XP3",
    extensions: &["xp3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
