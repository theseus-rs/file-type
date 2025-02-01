use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757953: FileFormat = FileFormat {
    id: 28_757_953,
    puid: "wikidata/28757953",
    name: "HGT",
    extensions: &["hgt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
