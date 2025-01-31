use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3008299: FileFormat = FileFormat {
    id: 3_008_299,
    puid: "wikidata/3008299",
    name: "xorg.conf",
    extensions: &["xorg.conf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
