use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125925206: FileFormat = FileFormat {
    id: 125_925_206,
    source_type: SourceType::Wikidata,
    name: "Papyrus Author database",
    extensions: &["pb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
