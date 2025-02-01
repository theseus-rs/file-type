use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111263298: FileFormat = FileFormat {
    id: 111_263_298,
    puid: "wikidata/111263298",
    name: "Digilink format",
    extensions: &["dig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
