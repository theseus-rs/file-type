use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_82065829: FileFormat = FileFormat {
    id: 82_065_829,
    source_type: SourceType::Wikidata,
    name: "ChiWriter high resolution screen font",
    extensions: &["eft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
