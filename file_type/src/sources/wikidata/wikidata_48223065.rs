use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48223065: FileFormat = FileFormat {
    id: 48_223_065,
    source_type: SourceType::Wikidata,
    name: "Turbo Debugger Keystroke Recording File",
    extensions: &["tdk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
