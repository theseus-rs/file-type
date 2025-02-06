use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127327949: FileFormat = FileFormat {
    id: 127_327_949,
    source_type: SourceType::Wikidata,
    name: "Coffeescript file",
    extensions: &["coffee"],
    media_types: &["text/coffeescript"],
    signatures: &[],
    related_formats: &[],
};
