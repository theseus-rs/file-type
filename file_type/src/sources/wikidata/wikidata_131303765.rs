use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131303765: FileFormat = FileFormat {
    id: 131_303_765,
    source_type: SourceType::Wikidata,
    name: "TL-b source code file",
    extensions: &["tlb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
