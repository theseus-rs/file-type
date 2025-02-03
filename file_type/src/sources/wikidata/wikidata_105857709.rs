use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857709: FileFormat = FileFormat {
    id: 105_857_709,
    source_type: SourceType::Wikidata,
    name: "Image Spectrumizer Workspace (v4.0)",
    extensions: &["isw"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
