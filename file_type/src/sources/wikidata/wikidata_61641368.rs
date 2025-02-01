use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61641368: FileFormat = FileFormat {
    id: 61_641_368,
    puid: "wikidata/61641368",
    name: "Microsoft Word for Windows Document, version 2",
    extensions: &["doc"],
    media_types: &["application/msword"],
    internal_signatures: &[],
    related_formats: &[],
};
