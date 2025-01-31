use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131147128: FileFormat = FileFormat {
    id: 131_147_128,
    puid: "wikidata/131147128",
    name: "Templated SQL file format",
    extensions: &["sql.j2", "sql.jinja2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
