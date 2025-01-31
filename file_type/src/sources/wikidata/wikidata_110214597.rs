use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110214597: FileFormat = FileFormat {
    id: 110_214_597,
    puid: "wikidata/110214597",
    name: "Serif PagePlus Publication file format, version X8",
    extensions: &["ppb", "ppp", "ppx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
