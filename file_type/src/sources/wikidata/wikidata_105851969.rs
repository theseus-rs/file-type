use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851969: FileFormat = FileFormat {
    id: 105_851_969,
    source_type: SourceType::Wikidata,
    name: "Sublime Text Mouse settings",
    extensions: &["sublime-mousemap"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
