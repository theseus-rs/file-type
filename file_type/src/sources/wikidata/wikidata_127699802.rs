use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127699802: FileFormat = FileFormat {
    id: 127_699_802,
    puid: "wikidata/127699802",
    name: "Futhark file",
    extensions: &["fut"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
