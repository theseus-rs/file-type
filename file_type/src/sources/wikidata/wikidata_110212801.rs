use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110212801: FileFormat = FileFormat {
    id: 110_212_801,
    puid: "wikidata/110212801",
    name: "Serif PagePlus Publication file format, version X5",
    extensions: &["ppb", "ppp", "ppx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
