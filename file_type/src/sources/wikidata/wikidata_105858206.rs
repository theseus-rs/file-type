use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858206: FileFormat = FileFormat {
    id: 105_858_206,
    source_type: SourceType::Wikidata,
    name: "IAR Embedded Workbench Workspace",
    extensions: &["eww"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
