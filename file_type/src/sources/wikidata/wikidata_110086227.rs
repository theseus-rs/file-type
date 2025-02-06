use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110086227: FileFormat = FileFormat {
    id: 110_086_227,
    source_type: SourceType::Wikidata,
    name: "NTI JewelCase Maker format",
    extensions: &["jwc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
