use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757979: FileFormat = FileFormat {
    id: 28_757_979,
    source_type: SourceType::Wikidata,
    name: "Windows Setup inf_loc file",
    extensions: &["inf_loc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
