use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110213247: FileFormat = FileFormat {
    id: 110_213_247,
    puid: "wikidata/110213247",
    name: "Serif PagePlus Publication file format, version X6",
    extensions: &["ppb", "ppp", "ppx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
