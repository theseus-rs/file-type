use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113291185: FileFormat = FileFormat {
    id: 113_291_185,
    puid: "wikidata/113291185",
    name: "Serif Metafile Format",
    extensions: &["smf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
