use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851969: FileFormat = FileFormat {
    id: 105_851_969,
    puid: "wikidata/105851969",
    name: "Sublime Text Mouse settings",
    extensions: &["sublime-mousemap"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
