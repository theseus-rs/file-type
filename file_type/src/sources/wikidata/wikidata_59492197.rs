use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59492197: FileFormat = FileFormat {
    id: 59_492_197,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System Catalog (Unix)",
    extensions: &["sas7bcat", "sc7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
