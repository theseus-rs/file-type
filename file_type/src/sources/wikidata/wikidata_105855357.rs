use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855357: FileFormat = FileFormat {
    id: 105_855_357,
    source_type: SourceType::Wikidata,
    name: "Flatpack Reference (with rem)",
    extensions: &["flatpakref"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
