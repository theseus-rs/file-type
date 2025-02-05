use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959810: FileFormat = FileFormat {
    id: 27_959_810,
    source_type: SourceType::Wikidata,
    name: "Ableton Live Set",
    extensions: &["als"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
