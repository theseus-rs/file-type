use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27959886: FileFormat = FileFormat {
    id: 27_959_886,
    puid: "wikidata/27959886",
    name: "Cubase song",
    extensions: &["all"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
