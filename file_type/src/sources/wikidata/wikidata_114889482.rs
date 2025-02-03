use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114889482: FileFormat = FileFormat {
    id: 114_889_482,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Fabric Calendar file",
    extensions: &["sfc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
