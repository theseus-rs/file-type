use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118289158: FileFormat = FileFormat {
    id: 118_289_158,
    puid: "wikidata/118289158",
    name: "Collection File",
    extensions: &["cfs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
