use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111341823: FileFormat = FileFormat {
    id: 111_341_823,
    puid: "wikidata/111341823",
    name: "Signed dwords (32-bit) data",
    extensions: &["sdw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
