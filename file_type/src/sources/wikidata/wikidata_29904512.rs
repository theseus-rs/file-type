use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29904512: FileFormat = FileFormat {
    id: 29_904_512,
    source_type: SourceType::Wikidata,
    name: "SAR",
    extensions: &["sar"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
