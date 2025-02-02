use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122947259: FileFormat = FileFormat {
    id: 122_947_259,
    source_type: SourceType::Wikidata,
    name: "Windows Enhanced Metafile, version 2.0",
    extensions: &["emf", "emz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
