use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114877040: FileFormat = FileFormat {
    id: 114_877_040,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money Backup File",
    extensions: &["mbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
