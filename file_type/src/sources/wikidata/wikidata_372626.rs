use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_372626: FileFormat = FileFormat {
    id: 372_626,
    source_type: SourceType::Wikidata,
    name: "Theora",
    extensions: &["ogg", "ogv"],
    media_types: &["video/theora"],
    signatures: &[],
    related_formats: &[],
};
