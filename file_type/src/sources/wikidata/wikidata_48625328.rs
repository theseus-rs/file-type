use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48625328: FileFormat = FileFormat {
    id: 48_625_328,
    puid: "wikidata/48625328",
    name: "Encapsulated PostScript, v2",
    extensions: &["eps", "epsf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
