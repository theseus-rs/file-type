use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852854: FileFormat = FileFormat {
    id: 105_852_854,
    source_type: SourceType::Wikidata,
    name: "Sublime Text Project",
    extensions: &["sublime-project"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
