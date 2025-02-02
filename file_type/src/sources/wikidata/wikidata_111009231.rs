use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111009231: FileFormat = FileFormat {
    id: 111_009_231,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Poster File format",
    extensions: &["sig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
