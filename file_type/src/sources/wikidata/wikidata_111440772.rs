use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111440772: FileFormat = FileFormat {
    id: 111_440_772,
    source_type: SourceType::Wikidata,
    name: "Ruby Script",
    extensions: &["rbw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
