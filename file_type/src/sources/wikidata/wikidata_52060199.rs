use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52060199: FileFormat = FileFormat {
    id: 52_060_199,
    source_type: SourceType::Wikidata,
    name: "Paint Shop Pro Image, version 7",
    extensions: &["psp", "pspimage"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
