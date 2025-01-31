use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131179431: FileFormat = FileFormat {
    id: 131_179_431,
    puid: "wikidata/131179431",
    name: "TableGen file format",
    extensions: &["td"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
