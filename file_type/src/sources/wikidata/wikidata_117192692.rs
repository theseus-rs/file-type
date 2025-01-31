use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117192692: FileFormat = FileFormat {
    id: 117_192_692,
    puid: "wikidata/117192692",
    name: "Photoshop Raw",
    extensions: &["raw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
