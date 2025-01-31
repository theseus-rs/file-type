use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51913488: FileFormat = FileFormat {
    id: 51_913_488,
    puid: "wikidata/51913488",
    name: "Fractal Design Painter RIFF Bitmap Graphics",
    extensions: &["rif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
