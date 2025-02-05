use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117834959: FileFormat = FileFormat {
    id: 117_834_959,
    source_type: SourceType::Wikidata,
    name: "Brooktrout Fax-Mail 96 file",
    extensions: &["brk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
