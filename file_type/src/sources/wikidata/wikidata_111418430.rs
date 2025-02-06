use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111418430: FileFormat = FileFormat {
    id: 111_418_430,
    source_type: SourceType::Wikidata,
    name: "Adobe Bridge Collection File",
    extensions: &["collection"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
