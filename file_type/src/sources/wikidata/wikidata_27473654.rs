use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27473654: FileFormat = FileFormat {
    id: 27_473_654,
    puid: "wikidata/27473654",
    name: "Band Interleaved by Pixel Image File",
    extensions: &["bip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
