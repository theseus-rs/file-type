use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48223065: FileFormat = FileFormat {
    id: 48_223_065,
    puid: "wikidata/48223065",
    name: "Turbo Debugger Keystroke Recording File",
    extensions: &["tdk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
