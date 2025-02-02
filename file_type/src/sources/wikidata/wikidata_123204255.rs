use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123204255: FileFormat = FileFormat {
    id: 123_204_255,
    source_type: SourceType::Wikidata,
    name: "Avid Media Composer Script",
    extensions: &["avc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
