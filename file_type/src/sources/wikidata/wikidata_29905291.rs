use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29905291: FileFormat = FileFormat {
    id: 29_905_291,
    source_type: SourceType::Wikidata,
    name: "Self-Extracting Archive",
    extensions: &["sfx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
