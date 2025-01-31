use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126194534: FileFormat = FileFormat {
    id: 126_194_534,
    puid: "wikidata/126194534",
    name: "MySQL View Definition Format",
    extensions: &["frm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
