use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205788: FileFormat = FileFormat {
    id: 28_205_788,
    puid: "wikidata/28205788",
    name: "Compact Picture Format",
    extensions: &["cpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
