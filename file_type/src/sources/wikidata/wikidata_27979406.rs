use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979406: FileFormat = FileFormat {
    id: 27_979_406,
    puid: "wikidata/27979406",
    name: "QTL",
    extensions: &["qtl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
