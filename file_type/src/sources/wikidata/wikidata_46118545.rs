use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_46118545: FileFormat = FileFormat {
    id: 46_118_545,
    puid: "wikidata/46118545",
    name: "Lotus Approach View File",
    extensions: &["apt"],
    media_types: &["application/vnd.lotus-approach"],
    internal_signatures: &[],
    related_formats: &[],
};
