use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117844169: FileFormat = FileFormat {
    id: 117_844_169,
    puid: "wikidata/117844169",
    name: "Kofax Group 4 file",
    extensions: &["kfx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
