use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114889482: FileFormat = FileFormat {
    id: 114_889_482,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Fabric Calendar file",
    extensions: &["sfc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
