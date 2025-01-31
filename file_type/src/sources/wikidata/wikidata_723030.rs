use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_723030: FileFormat = FileFormat {
    id: 723_030,
    puid: "wikidata/723030",
    name: "AsciiDoc",
    extensions: &["adoc", "asciidoc", "txt"],
    media_types: &["text/asciidoc", "text/asciidoc", "text/asciidoc"],
    internal_signatures: &[],
    related_formats: &[],
};
