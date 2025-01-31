use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111600974: FileFormat = FileFormat {
    id: 111_600_974,
    puid: "wikidata/111600974",
    name: "Adobe InDesign Document, version CC 2015",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
