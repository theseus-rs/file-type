use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979383: FileFormat = FileFormat {
    id: 27_979_383,
    puid: "wikidata/27979383",
    name: "Heroglyph Project Format",
    extensions: &["hprj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
