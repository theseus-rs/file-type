use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28551284: FileFormat = FileFormat {
    id: 28_551_284,
    source_type: SourceType::Wikidata,
    name: "Adobe CMYK Setup File",
    extensions: &["api"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
