use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50375294: FileFormat = FileFormat {
    id: 50_375_294,
    source_type: SourceType::Wikidata,
    name: "ESRI ArcScene Document",
    extensions: &["sxd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
