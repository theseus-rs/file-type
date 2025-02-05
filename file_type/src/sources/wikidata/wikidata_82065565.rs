use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_82065565: FileFormat = FileFormat {
    id: 82_065_565,
    source_type: SourceType::Wikidata,
    name: "PhotoShop Extended Digital Book",
    extensions: &["edb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
