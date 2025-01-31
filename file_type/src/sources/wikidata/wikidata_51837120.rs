use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51837120: FileFormat = FileFormat {
    id: 51_837_120,
    puid: "wikidata/51837120",
    name: "Scitex Continuous Tone Bitmap",
    extensions: &["ct"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
