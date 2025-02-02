use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116378918: FileFormat = FileFormat {
    id: 116_378_918,
    source_type: SourceType::Wikidata,
    name: "Approach Database File",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
