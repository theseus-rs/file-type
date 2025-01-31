use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127691331: FileFormat = FileFormat {
    id: 127_691_331,
    puid: "wikidata/127691331",
    name: "Dylan source code file",
    extensions: &["dylan"],
    media_types: &["text/x-dylan"],
    internal_signatures: &[],
    related_formats: &[],
};
