use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757958: FileFormat = FileFormat {
    id: 28_757_958,
    puid: "wikidata/28757958",
    name: "Hangul Word Processor Document",
    extensions: &["hwp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
