use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119786627: FileFormat = FileFormat {
    id: 119_786_627,
    source_type: SourceType::Wikidata,
    name: "Export v4 File",
    extensions: &["mxp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
