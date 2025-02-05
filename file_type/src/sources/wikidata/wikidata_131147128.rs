use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131147128: FileFormat = FileFormat {
    id: 131_147_128,
    source_type: SourceType::Wikidata,
    name: "Templated SQL file format",
    extensions: &["sql.j2", "sql.jinja2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
