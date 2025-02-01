use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127605519: FileFormat = FileFormat {
    id: 127_605_519,
    puid: "wikidata/127605519",
    name: "Crystal file format",
    extensions: &["cr"],
    media_types: &["text/x-crystal"],
    internal_signatures: &[],
    related_formats: &[],
};
