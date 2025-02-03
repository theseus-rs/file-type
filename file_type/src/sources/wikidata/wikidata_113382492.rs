use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113382492: FileFormat = FileFormat {
    id: 113_382_492,
    source_type: SourceType::Wikidata,
    name: "Roxio Easy Media Creator Classic Creator File 8-10",
    extensions: &["rcl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
