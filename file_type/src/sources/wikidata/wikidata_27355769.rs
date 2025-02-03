use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27355769: FileFormat = FileFormat {
    id: 27_355_769,
    source_type: SourceType::Wikidata,
    name: "ADRG Legend Image File",
    extensions: &["lgg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
