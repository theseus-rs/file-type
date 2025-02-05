use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207058: FileFormat = FileFormat {
    id: 28_207_058,
    source_type: SourceType::Wikidata,
    name: "Poser Bump Map",
    extensions: &["bum"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
