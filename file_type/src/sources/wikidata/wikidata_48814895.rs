use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48814895: FileFormat = FileFormat {
    id: 48_814_895,
    source_type: SourceType::Wikidata,
    name: "ESRI ArcView Project",
    extensions: &["apr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
