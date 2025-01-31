use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50499145: FileFormat = FileFormat {
    id: 50_499_145,
    puid: "wikidata/50499145",
    name: "QuickDraw 3D Metafile (ASCII)",
    extensions: &["3dmf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
