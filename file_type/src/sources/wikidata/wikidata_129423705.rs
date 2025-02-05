use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129423705: FileFormat = FileFormat {
    id: 129_423_705,
    source_type: SourceType::Wikidata,
    name: "Gosu source code file",
    extensions: &["gs"],
    media_types: &["text/x-gosu"],
    signatures: &[],
    related_formats: &[],
};
