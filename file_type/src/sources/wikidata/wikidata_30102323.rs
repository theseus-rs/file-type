use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_30102323: FileFormat = FileFormat {
    id: 30_102_323,
    source_type: SourceType::Wikidata,
    name: "Amateur Data Interchange Format, ADI variant, version 3.0.5",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
