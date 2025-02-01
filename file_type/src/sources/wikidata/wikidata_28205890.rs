use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205890: FileFormat = FileFormat {
    id: 28_205_890,
    puid: "wikidata/28205890",
    name: "Multipage Z-Soft Paintbrush Bitmap Graphics",
    extensions: &["dcx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
