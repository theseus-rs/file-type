use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_65990344: FileFormat = FileFormat {
    id: 65_990_344,
    source_type: SourceType::Wikidata,
    name: "Adobe Premiere Project",
    extensions: &["ppj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
