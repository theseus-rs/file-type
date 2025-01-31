use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111596697: FileFormat = FileFormat {
    id: 111_596_697,
    puid: "wikidata/111596697",
    name: "Adobe InDesign Document, version 2",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
