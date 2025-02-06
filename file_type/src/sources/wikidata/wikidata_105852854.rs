use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852854: FileFormat = FileFormat {
    id: 105_852_854,
    source_type: SourceType::Wikidata,
    name: "Sublime Text Project",
    extensions: &["sublime-project"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
