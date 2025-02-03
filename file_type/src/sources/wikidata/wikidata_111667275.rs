use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111667275: FileFormat = FileFormat {
    id: 111_667_275,
    source_type: SourceType::Wikidata,
    name: "OEW objectbase",
    extensions: &["oew"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
