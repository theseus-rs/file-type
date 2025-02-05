use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111009348: FileFormat = FileFormat {
    id: 111_009_348,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Brochure File format",
    extensions: &["bro"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
