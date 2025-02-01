use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_46118194: FileFormat = FileFormat {
    id: 46_118_194,
    puid: "wikidata/46118194",
    name: "Lotus Approach View File, version 97",
    extensions: &["apr"],
    media_types: &["application/vnd.lotus-approach"],
    internal_signatures: &[],
    related_formats: &[],
};
