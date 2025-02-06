use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109971781: FileFormat = FileFormat {
    id: 109_971_781,
    source_type: SourceType::Wikidata,
    name: "PDF Portfolio file format",
    extensions: &["pdf"],
    media_types: &["application/pdf"],
    signatures: &[],
    related_formats: &[],
};
