use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116378812: FileFormat = FileFormat {
    id: 116_378_812,
    source_type: SourceType::Wikidata,
    name: "Act! Database File",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
