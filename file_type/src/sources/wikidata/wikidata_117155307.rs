use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117155307: FileFormat = FileFormat {
    id: 117_155_307,
    puid: "wikidata/117155307",
    name: "Picture It! PNG Plus",
    extensions: &["png"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
