use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111333291: FileFormat = FileFormat {
    id: 111_333_291,
    puid: "wikidata/111333291",
    name: "DisorderTracker2 sample",
    extensions: &["pls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
