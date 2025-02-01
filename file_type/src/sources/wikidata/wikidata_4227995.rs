use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4227995: FileFormat = FileFormat {
    id: 4_227_995,
    puid: "wikidata/4227995",
    name: "eMule collection",
    extensions: &["emulecollection"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
