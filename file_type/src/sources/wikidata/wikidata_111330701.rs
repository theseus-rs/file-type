use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111330701: FileFormat = FileFormat {
    id: 111_330_701,
    source_type: SourceType::Wikidata,
    name: "MadTracker 2 instruments",
    extensions: &["mti"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
