use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111440772: FileFormat = FileFormat {
    id: 111_440_772,
    source_type: SourceType::Wikidata,
    name: "Ruby Script",
    extensions: &["rbw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
