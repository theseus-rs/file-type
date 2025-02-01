use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110213914: FileFormat = FileFormat {
    id: 110_213_914,
    puid: "wikidata/110213914",
    name: "Serif PagePlus Publication file format, version X7",
    extensions: &["ppb", "ppp", "ppx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
