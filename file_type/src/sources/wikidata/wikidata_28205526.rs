use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205526: FileFormat = FileFormat {
    id: 28_205_526,
    puid: "wikidata/28205526",
    name: "Icon library",
    extensions: &["icl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
