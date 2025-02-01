use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205708: FileFormat = FileFormat {
    id: 28_205_708,
    puid: "wikidata/28205708",
    name: "Applixware Bitmap",
    extensions: &["im"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
