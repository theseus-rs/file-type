use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122168574: FileFormat = FileFormat {
    id: 122_168_574,
    source_type: SourceType::Wikidata,
    name: "Old Security Explorer Project",
    extensions: &["nsp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
