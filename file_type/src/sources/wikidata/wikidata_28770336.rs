use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28770336: FileFormat = FileFormat {
    id: 28_770_336,
    puid: "wikidata/28770336",
    name: "Lotus 1-2-3 Chart",
    extensions: &["pic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
