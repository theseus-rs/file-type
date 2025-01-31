use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857709: FileFormat = FileFormat {
    id: 105_857_709,
    puid: "wikidata/105857709",
    name: "Image Spectrumizer Workspace (v4.0)",
    extensions: &["isw"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
