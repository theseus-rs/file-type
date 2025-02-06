use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129498387: FileFormat = FileFormat {
    id: 129_498_387,
    source_type: SourceType::Wikidata,
    name: "Haml file format",
    extensions: &["haml"],
    media_types: &["text/x-haml"],
    signatures: &[],
    related_formats: &[],
};
