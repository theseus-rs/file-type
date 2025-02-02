use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111009348: FileFormat = FileFormat {
    id: 111_009_348,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Brochure File format",
    extensions: &["bro"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
