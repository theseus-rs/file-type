use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206916: FileFormat = FileFormat {
    id: 28_206_916,
    source_type: SourceType::Wikidata,
    name: "Portfolio Graphics",
    extensions: &["pgf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
