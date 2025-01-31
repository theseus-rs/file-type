use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111606210: FileFormat = FileFormat {
    id: 111_606_210,
    puid: "wikidata/111606210",
    name: "Adobe InDesign Document, version CC 2021",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
