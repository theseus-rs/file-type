use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59715886: FileFormat = FileFormat {
    id: 59_715_886,
    puid: "wikidata/59715886",
    name: "CALS Compressed Bitmap",
    extensions: &["cal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
