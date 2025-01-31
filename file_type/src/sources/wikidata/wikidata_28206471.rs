use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206471: FileFormat = FileFormat {
    id: 28_206_471,
    puid: "wikidata/28206471",
    name: "KoalaPainter compressed",
    extensions: &["gg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
