use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124843932: FileFormat = FileFormat {
    id: 124_843_932,
    puid: "wikidata/124843932",
    name: "Apple Contacts Archive file",
    extensions: &["abbu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
