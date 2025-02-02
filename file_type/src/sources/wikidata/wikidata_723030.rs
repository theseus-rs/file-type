use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_723030: FileFormat = FileFormat {
    id: 723_030,
    source_type: SourceType::Wikidata,
    name: "AsciiDoc",
    extensions: &["adoc", "asciidoc", "txt"],
    media_types: &["text/asciidoc"],
    internal_signatures: &[],
    related_formats: &[],
};
