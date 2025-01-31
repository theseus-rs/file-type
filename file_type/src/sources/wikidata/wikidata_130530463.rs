use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130530463: FileFormat = FileFormat {
    id: 130_530_463,
    puid: "wikidata/130530463",
    name: "Praat script file",
    extensions: &["praat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
