use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130266833: FileFormat = FileFormat {
    id: 130_266_833,
    puid: "wikidata/130266833",
    name: "Macaulay2 file format",
    extensions: &["m2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
