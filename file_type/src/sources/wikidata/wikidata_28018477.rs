use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28018477: FileFormat = FileFormat {
    id: 28_018_477,
    puid: "wikidata/28018477",
    name: "Indeo Video Format",
    extensions: &["ivf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
