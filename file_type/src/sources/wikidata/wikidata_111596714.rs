use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111596714: FileFormat = FileFormat {
    id: 111_596_714,
    puid: "wikidata/111596714",
    name: "Adobe InDesign Document, version CS 5.5",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
