use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34745947: FileFormat = FileFormat {
    id: 34_745_947,
    source_type: SourceType::Wikidata,
    name: "Starlink Data Format",
    extensions: &["sdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
