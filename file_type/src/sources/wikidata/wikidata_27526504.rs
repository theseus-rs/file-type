use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27526504: FileFormat = FileFormat {
    id: 27_526_504,
    puid: "wikidata/27526504",
    name: "Broadcast Wave Format, version 2",
    extensions: &["wav"],
    media_types: &["audio/x-wav"],
    internal_signatures: &[],
    related_formats: &[],
};
