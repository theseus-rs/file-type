use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122248584: FileFormat = FileFormat {
    id: 122_248_584,
    puid: "wikidata/122248584",
    name: "YUV Video File",
    extensions: &["yuv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
