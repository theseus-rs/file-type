use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29904512: FileFormat = FileFormat {
    id: 29_904_512,
    source_type: SourceType::Wikidata,
    name: "SAR",
    extensions: &["sar"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
