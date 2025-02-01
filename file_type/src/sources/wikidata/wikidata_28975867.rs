use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975867: FileFormat = FileFormat {
    id: 28_975_867,
    puid: "wikidata/28975867",
    name: "OOGL SKEL file",
    extensions: &["skel"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
