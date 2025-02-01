use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967183: FileFormat = FileFormat {
    id: 27_967_183,
    puid: "wikidata/27967183",
    name: "FastTracker module",
    extensions: &["ft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
