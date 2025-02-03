use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2931409: FileFormat = FileFormat {
    id: 2_931_409,
    source_type: SourceType::Wikidata,
    name: "CFD General Notation System",
    extensions: &["cgns"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
