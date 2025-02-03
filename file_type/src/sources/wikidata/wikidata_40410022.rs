use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_40410022: FileFormat = FileFormat {
    id: 40_410_022,
    source_type: SourceType::Wikidata,
    name: "Feather",
    extensions: &["feather"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
