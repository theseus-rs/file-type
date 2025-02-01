use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_10387757: FileFormat = FileFormat {
    id: 10_387_757,
    puid: "wikidata/10387757",
    name: "Universal Image Format",
    extensions: &["uif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
