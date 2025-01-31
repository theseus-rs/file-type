use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123385688: FileFormat = FileFormat {
    id: 123_385_688,
    puid: "wikidata/123385688",
    name: "iSpace 1.0 Scene file",
    extensions: &["iss"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
