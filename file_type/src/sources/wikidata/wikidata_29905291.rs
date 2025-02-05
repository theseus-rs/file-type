use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29905291: FileFormat = FileFormat {
    id: 29_905_291,
    source_type: SourceType::Wikidata,
    name: "Self-Extracting Archive",
    extensions: &["sfx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
