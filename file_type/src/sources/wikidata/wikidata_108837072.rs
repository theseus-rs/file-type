use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_108837072: FileFormat = FileFormat {
    id: 108_837_072,
    source_type: SourceType::Wikidata,
    name: "Nero HFS-CD Compilation",
    extensions: &["nhf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
