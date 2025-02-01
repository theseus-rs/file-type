use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51839234: FileFormat = FileFormat {
    id: 51_839_234,
    puid: "wikidata/51839234",
    name: "Inset Systems Bitmap",
    extensions: &["pix"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
