use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5636096: FileFormat = FileFormat {
    id: 5_636_096,
    puid: "wikidata/5636096",
    name: "HTML Components",
    extensions: &["htc"],
    media_types: &["text/x-component"],
    internal_signatures: &[],
    related_formats: &[],
};
