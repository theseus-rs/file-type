use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967417: FileFormat = FileFormat {
    id: 27_967_417,
    source_type: SourceType::Wikidata,
    name: "Callus OPL Register Log",
    extensions: &["cym"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
