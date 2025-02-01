use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50223857: FileFormat = FileFormat {
    id: 50_223_857,
    puid: "wikidata/50223857",
    name: "ESRI ArcMap Document",
    extensions: &["mxd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
