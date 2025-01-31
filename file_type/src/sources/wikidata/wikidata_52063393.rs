use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52063393: FileFormat = FileFormat {
    id: 52_063_393,
    puid: "wikidata/52063393",
    name: "TeX Binary File",
    extensions: &["dvi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
