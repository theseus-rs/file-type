use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_57978083: FileFormat = FileFormat {
    id: 57_978_083,
    source_type: SourceType::Wikidata,
    name: "ASP Application Directive File",
    extensions: &["asax"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
