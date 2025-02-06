use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111009602: FileFormat = FileFormat {
    id: 111_009_602,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Business Card File format",
    extensions: &["biz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
