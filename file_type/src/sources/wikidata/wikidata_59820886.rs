use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59820886: FileFormat = FileFormat {
    id: 59_820_886,
    puid: "wikidata/59820886",
    name: "Corel CMX Compressed",
    extensions: &["cpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
