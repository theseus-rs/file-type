use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110086227: FileFormat = FileFormat {
    id: 110_086_227,
    source_type: SourceType::Wikidata,
    name: "NTI JewelCase Maker format",
    extensions: &["jwc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
