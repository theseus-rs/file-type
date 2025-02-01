use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852854: FileFormat = FileFormat {
    id: 105_852_854,
    puid: "wikidata/105852854",
    name: "Sublime Text Project",
    extensions: &["sublime-project"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
