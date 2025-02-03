use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125134354: FileFormat = FileFormat {
    id: 125_134_354,
    source_type: SourceType::Wikidata,
    name: "YAM Folder Configuration",
    extensions: &["fconfig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
