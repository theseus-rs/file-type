use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207058: FileFormat = FileFormat {
    id: 28_207_058,
    source_type: SourceType::Wikidata,
    name: "Poser Bump Map",
    extensions: &["bum"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
