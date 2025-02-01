use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867269: FileFormat = FileFormat {
    id: 105_867_269,
    puid: "wikidata/105867269",
    name: "OS/2 Network Information File",
    extensions: &["nif"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
