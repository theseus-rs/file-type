use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_6934775: FileFormat = FileFormat {
    id: 6_934_775,
    puid: "wikidata/6934775",
    name: "Multimedia Container Format",
    extensions: &["audio.mcf", "av.mcf", "mcf", "video.mcf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
