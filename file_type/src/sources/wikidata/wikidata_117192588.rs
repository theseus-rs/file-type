use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117192588: FileFormat = FileFormat {
    id: 117_192_588,
    source_type: SourceType::Wikidata,
    name: "Photoshop PDF",
    extensions: &["pdf", "pdp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
