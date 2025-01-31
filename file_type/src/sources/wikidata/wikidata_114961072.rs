use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114961072: FileFormat = FileFormat {
    id: 114_961_072,
    puid: "wikidata/114961072",
    name: "Writer's DreamKit 4.0 file",
    extensions: &["dsw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
