use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27350220: FileFormat = FileFormat {
    id: 27_350_220,
    source_type: SourceType::Wikidata,
    name: "ADRG General Information File",
    extensions: &["gen"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
