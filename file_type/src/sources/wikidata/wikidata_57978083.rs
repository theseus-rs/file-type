use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_57978083: FileFormat = FileFormat {
    id: 57_978_083,
    puid: "wikidata/57978083",
    name: "ASP Application Directive File",
    extensions: &["asax"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
