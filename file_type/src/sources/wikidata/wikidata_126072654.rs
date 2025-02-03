use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126072654: FileFormat = FileFormat {
    id: 126_072_654,
    source_type: SourceType::Wikidata,
    name: "WinFax Sent / Received Document file",
    extensions: &["fxr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
