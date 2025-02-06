use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50376371: FileFormat = FileFormat {
    id: 50_376_371,
    source_type: SourceType::Wikidata,
    name: "SHA256 File",
    extensions: &["sha256"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
