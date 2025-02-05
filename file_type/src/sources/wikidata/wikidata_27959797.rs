use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959797: FileFormat = FileFormat {
    id: 27_959_797,
    source_type: SourceType::Wikidata,
    name: "Ableton Device Preset",
    extensions: &["adp", "adv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
