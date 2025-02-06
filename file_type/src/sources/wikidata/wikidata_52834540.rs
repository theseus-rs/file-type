use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52834540: FileFormat = FileFormat {
    id: 52_834_540,
    source_type: SourceType::Wikidata,
    name: "Paint Shop Pro Image, version 4",
    extensions: &["psp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
