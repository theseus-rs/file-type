use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109265753: FileFormat = FileFormat {
    id: 109_265_753,
    puid: "wikidata/109265753",
    name: "PagePlus Template",
    extensions: &["ppt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
