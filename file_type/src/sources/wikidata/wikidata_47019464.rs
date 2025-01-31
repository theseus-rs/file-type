use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47019464: FileFormat = FileFormat {
    id: 47_019_464,
    puid: "wikidata/47019464",
    name: "PageMaker Document file format, version 5",
    extensions: &["pm5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
