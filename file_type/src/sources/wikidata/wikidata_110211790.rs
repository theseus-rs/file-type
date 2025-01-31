use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110211790: FileFormat = FileFormat {
    id: 110_211_790,
    puid: "wikidata/110211790",
    name: "Serif PagePlus Publication file format, version X4",
    extensions: &["ppb", "ppp", "ppx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
