use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110215299: FileFormat = FileFormat {
    id: 110_215_299,
    puid: "wikidata/110215299",
    name: "Serif PagePlus Publication file format, version X9",
    extensions: &["ppb", "ppp", "ppx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
