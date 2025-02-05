use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205468: FileFormat = FileFormat {
    id: 28_205_468,
    source_type: SourceType::Wikidata,
    name: "Sony Mavica 411",
    extensions: &["411"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
