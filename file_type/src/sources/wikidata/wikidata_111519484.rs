use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111519484: FileFormat = FileFormat {
    id: 111_519_484,
    source_type: SourceType::Wikidata,
    name: "ESRI ArcInfo Grid .nit File",
    extensions: &["nit"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
