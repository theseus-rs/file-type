use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28777714: FileFormat = FileFormat {
    id: 28_777_714,
    source_type: SourceType::Wikidata,
    name: "NII",
    extensions: &["nii"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
