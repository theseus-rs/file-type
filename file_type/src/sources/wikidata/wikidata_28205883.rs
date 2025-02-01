use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205883: FileFormat = FileFormat {
    id: 28_205_883,
    puid: "wikidata/28205883",
    name: "Desktop Color Separation",
    extensions: &["c", "dcs", "eps", "k", "m", "y"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
