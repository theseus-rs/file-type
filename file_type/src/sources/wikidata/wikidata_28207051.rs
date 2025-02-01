use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207051: FileFormat = FileFormat {
    id: 28_207_051,
    puid: "wikidata/28207051",
    name: "Pocket PC Bitmap",
    extensions: &["2bp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
