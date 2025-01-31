use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5301: FileFormat = FileFormat {
    id: 5_301,
    puid: "wikidata/5301",
    name: "TeX",
    extensions: &["tex", "tex", "tex"],
    media_types: &["application/x-tex", "math/tex", "text/x-tex"],
    internal_signatures: &[],
    related_formats: &[],
};
