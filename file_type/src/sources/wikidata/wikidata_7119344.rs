use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7119344: FileFormat = FileFormat {
    id: 7_119_344,
    puid: "wikidata/7119344",
    name: "PICtor PIC image format",
    extensions: &["clp", "pic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
