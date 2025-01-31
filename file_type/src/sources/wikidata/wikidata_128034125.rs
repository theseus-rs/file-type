use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128034125: FileFormat = FileFormat {
    id: 128_034_125,
    puid: "wikidata/128034125",
    name: "Rexx source code file",
    extensions: &["arexx", "rex", "rexx", "rx"],
    media_types: &["text/x-rexx", "text/x-rexx", "text/x-rexx", "text/x-rexx"],
    internal_signatures: &[],
    related_formats: &[],
};
