use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47018470: FileFormat = FileFormat {
    id: 47_018_470,
    source_type: SourceType::Wikidata,
    name: "PageMaker Document file format, version 4",
    extensions: &["pm4"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
