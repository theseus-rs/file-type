use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111009727: FileFormat = FileFormat {
    id: 111_009_727,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Envelope File format",
    extensions: &["env"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
