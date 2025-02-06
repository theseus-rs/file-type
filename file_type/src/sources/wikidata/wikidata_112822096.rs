use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112822096: FileFormat = FileFormat {
    id: 112_822_096,
    source_type: SourceType::Wikidata,
    name: "Strata StudioPro 3D File, version 1.75",
    extensions: &["vis"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
