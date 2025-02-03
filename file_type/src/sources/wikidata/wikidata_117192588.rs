use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117192588: FileFormat = FileFormat {
    id: 117_192_588,
    source_type: SourceType::Wikidata,
    name: "Photoshop PDF",
    extensions: &["pdf", "pdp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
