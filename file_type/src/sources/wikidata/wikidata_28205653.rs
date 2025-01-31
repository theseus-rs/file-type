use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205653: FileFormat = FileFormat {
    id: 28_205_653,
    puid: "wikidata/28205653",
    name: "Abekas YUV",
    extensions: &["yuv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
