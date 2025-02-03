use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_57978083: FileFormat = FileFormat {
    id: 57_978_083,
    source_type: SourceType::Wikidata,
    name: "ASP Application Directive File",
    extensions: &["asax"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
