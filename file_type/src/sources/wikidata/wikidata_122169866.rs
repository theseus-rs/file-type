use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122169866: FileFormat = FileFormat {
    id: 122_169_866,
    puid: "wikidata/122169866",
    name: "Lotus Notes User ID File",
    extensions: &["id"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
