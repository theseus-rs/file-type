use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50223857: FileFormat = FileFormat {
    id: 50_223_857,
    source_type: SourceType::Wikidata,
    name: "ESRI ArcMap Document",
    extensions: &["mxd"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
