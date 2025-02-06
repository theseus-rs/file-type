use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_30102323: FileFormat = FileFormat {
    id: 30_102_323,
    source_type: SourceType::Wikidata,
    name: "Amateur Data Interchange Format, ADI variant, version 3.0.5",
    extensions: &["adi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
