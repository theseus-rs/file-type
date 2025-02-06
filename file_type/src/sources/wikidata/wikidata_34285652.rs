use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34285652: FileFormat = FileFormat {
    id: 34_285_652,
    source_type: SourceType::Wikidata,
    name: "Perl Common Gateway Interface script",
    extensions: &["cgi", "fcgi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
