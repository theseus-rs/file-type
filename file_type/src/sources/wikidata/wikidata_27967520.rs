use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967520: FileFormat = FileFormat {
    id: 27_967_520,
    puid: "wikidata/27967520",
    name: "Matroska 3D Stereoscopic video",
    extensions: &["mk3d"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
