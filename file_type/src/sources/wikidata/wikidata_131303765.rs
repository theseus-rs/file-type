use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131303765: FileFormat = FileFormat {
    id: 131_303_765,
    source_type: SourceType::Wikidata,
    name: "TL-b source code file",
    extensions: &["tlb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
