use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859442: FileFormat = FileFormat {
    id: 105_859_442,
    source_type: SourceType::Wikidata,
    name: "Q Light Controller+ Workspace",
    extensions: &["qxw"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
