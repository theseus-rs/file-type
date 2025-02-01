use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_288256: FileFormat = FileFormat {
    id: 288_256,
    puid: "wikidata/288256",
    name: "ACE",
    extensions: &["ace"],
    media_types: &["application/x-ace-compressed"],
    internal_signatures: &[],
    related_formats: &[],
};
