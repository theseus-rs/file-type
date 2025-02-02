use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112822096: FileFormat = FileFormat {
    id: 112_822_096,
    source_type: SourceType::Wikidata,
    name: "Strata StudioPro 3D File, version 1.75",
    extensions: &["vis"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
