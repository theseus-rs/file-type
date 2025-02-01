use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47018470: FileFormat = FileFormat {
    id: 47_018_470,
    puid: "wikidata/47018470",
    name: "PageMaker Document file format, version 4",
    extensions: &["pm4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
