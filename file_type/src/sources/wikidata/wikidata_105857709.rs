use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857709: FileFormat = FileFormat {
    id: 105_857_709,
    source_type: SourceType::Wikidata,
    name: "Image Spectrumizer Workspace (v4.0)",
    extensions: &["isw"],
    media_types: &["text/json"],
    signatures: &[],
    related_formats: &[],
};
