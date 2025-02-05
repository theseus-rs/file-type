use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959791: FileFormat = FileFormat {
    id: 27_959_791,
    source_type: SourceType::Wikidata,
    name: "Ableton Device Group",
    extensions: &["adg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
