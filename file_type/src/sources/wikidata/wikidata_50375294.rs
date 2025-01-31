use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50375294: FileFormat = FileFormat {
    id: 50_375_294,
    puid: "wikidata/50375294",
    name: "ESRI ArcScene Document",
    extensions: &["sxd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
