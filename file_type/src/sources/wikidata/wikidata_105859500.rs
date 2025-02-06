use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859500: FileFormat = FileFormat {
    id: 105_859_500,
    source_type: SourceType::Wikidata,
    name: "VSIX Manifest (2011)",
    extensions: &["vsixmanifest"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
