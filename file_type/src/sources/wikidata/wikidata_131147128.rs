use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131147128: FileFormat = FileFormat {
    id: 131_147_128,
    source_type: SourceType::Wikidata,
    name: "Templated SQL file format",
    extensions: &["sql.j2", "sql.jinja2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
