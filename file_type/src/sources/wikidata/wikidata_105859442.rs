use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859442: FileFormat = FileFormat {
    id: 105_859_442,
    source_type: SourceType::Wikidata,
    name: "Q Light Controller+ Workspace",
    extensions: &["qxw"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
