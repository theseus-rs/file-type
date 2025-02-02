use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858358: FileFormat = FileFormat {
    id: 105_858_358,
    source_type: SourceType::Wikidata,
    name: "IAR Embedded Workbench Project",
    extensions: &["ewp"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
