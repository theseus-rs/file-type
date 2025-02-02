use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852417: FileFormat = FileFormat {
    id: 105_852_417,
    source_type: SourceType::Wikidata,
    name: "Microsoft SQL Server execution Plan",
    extensions: &["sqlplan"],
    media_types: &["application/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
