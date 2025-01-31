use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859500: FileFormat = FileFormat {
    id: 105_859_500,
    puid: "wikidata/105859500",
    name: "VSIX Manifest (2011)",
    extensions: &["vsixmanifest"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
