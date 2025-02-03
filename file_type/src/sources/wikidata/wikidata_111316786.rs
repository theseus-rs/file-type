use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111316786: FileFormat = FileFormat {
    id: 111_316_786,
    source_type: SourceType::Wikidata,
    name: "KAWAI R50/R50E/R50III/R100 ROM-dump",
    extensions: &["kawai12"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
