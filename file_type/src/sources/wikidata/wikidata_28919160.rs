use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28919160: FileFormat = FileFormat {
    id: 28_919_160,
    puid: "wikidata/28919160",
    name: "Standard ACIS Binary",
    extensions: &["sab"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
