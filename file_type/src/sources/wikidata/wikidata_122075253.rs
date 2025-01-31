use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122075253: FileFormat = FileFormat {
    id: 122_075_253,
    puid: "wikidata/122075253",
    name: "Finale Template File",
    extensions: &["ftm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
