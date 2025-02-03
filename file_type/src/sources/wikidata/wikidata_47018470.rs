use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47018470: FileFormat = FileFormat {
    id: 47_018_470,
    source_type: SourceType::Wikidata,
    name: "PageMaker Document file format, version 4",
    extensions: &["pm4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
