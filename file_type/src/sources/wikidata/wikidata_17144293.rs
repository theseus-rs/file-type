use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_17144293: FileFormat = FileFormat {
    id: 17_144_293,
    puid: "wikidata/17144293",
    name: "UBJSON",
    extensions: &["ubj"],
    media_types: &["application/ubjson"],
    internal_signatures: &[],
    related_formats: &[],
};
