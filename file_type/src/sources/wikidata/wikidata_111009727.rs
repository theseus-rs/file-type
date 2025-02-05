use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111009727: FileFormat = FileFormat {
    id: 111_009_727,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Envelope File format",
    extensions: &["env"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
