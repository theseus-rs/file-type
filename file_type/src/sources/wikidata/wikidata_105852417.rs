use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852417: FileFormat = FileFormat {
    id: 105_852_417,
    puid: "wikidata/105852417",
    name: "Microsoft SQL Server execution Plan",
    extensions: &["sqlplan"],
    media_types: &["application/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
