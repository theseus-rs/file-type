use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111418430: FileFormat = FileFormat {
    id: 111_418_430,
    source_type: SourceType::Wikidata,
    name: "Adobe Bridge Collection File",
    extensions: &["collection"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
