use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851969: FileFormat = FileFormat {
    id: 105_851_969,
    source_type: SourceType::Wikidata,
    name: "Sublime Text Mouse settings",
    extensions: &["sublime-mousemap"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
