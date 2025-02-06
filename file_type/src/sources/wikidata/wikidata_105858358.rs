use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858358: FileFormat = FileFormat {
    id: 105_858_358,
    source_type: SourceType::Wikidata,
    name: "IAR Embedded Workbench Project",
    extensions: &["ewp"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
