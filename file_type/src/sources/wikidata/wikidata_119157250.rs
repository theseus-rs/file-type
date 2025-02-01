use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119157250: FileFormat = FileFormat {
    id: 119_157_250,
    puid: "wikidata/119157250",
    name: "Digital Image Publishing File",
    extensions: &["php"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
