use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110015313: FileFormat = FileFormat {
    id: 110_015_313,
    puid: "wikidata/110015313",
    name: "Autorun Maestro Menu File",
    extensions: &["mnu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
