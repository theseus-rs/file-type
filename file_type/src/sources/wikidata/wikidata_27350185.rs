use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27350185: FileFormat = FileFormat {
    id: 27_350_185,
    source_type: SourceType::Wikidata,
    name: "ADRG Test Patch Image File",
    extensions: &["cph"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
