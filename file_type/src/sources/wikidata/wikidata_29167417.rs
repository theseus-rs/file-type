use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29167417: FileFormat = FileFormat {
    id: 29_167_417,
    source_type: SourceType::Wikidata,
    name: "Folio",
    extensions: &["folio"],
    media_types: &["application/vnd.adobe.folio+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
