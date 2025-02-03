use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867269: FileFormat = FileFormat {
    id: 105_867_269,
    source_type: SourceType::Wikidata,
    name: "OS/2 Network Information File",
    extensions: &["nif"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
