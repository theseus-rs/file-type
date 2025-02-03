use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859500: FileFormat = FileFormat {
    id: 105_859_500,
    source_type: SourceType::Wikidata,
    name: "VSIX Manifest (2011)",
    extensions: &["vsixmanifest"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
