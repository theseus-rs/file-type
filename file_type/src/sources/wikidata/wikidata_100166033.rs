use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100166033: FileFormat = FileFormat {
    id: 100_166_033,
    puid: "wikidata/100166033",
    name: "Drawing Interchange Format (Binary) (Generic)",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
