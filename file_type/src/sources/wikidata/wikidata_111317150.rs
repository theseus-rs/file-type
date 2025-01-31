use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111317150: FileFormat = FileFormat {
    id: 111_317_150,
    puid: "wikidata/111317150",
    name: "Korg T-series wave",
    extensions: &["kft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
