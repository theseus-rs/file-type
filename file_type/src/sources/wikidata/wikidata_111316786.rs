use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111316786: FileFormat = FileFormat {
    id: 111_316_786,
    source_type: SourceType::Wikidata,
    name: "KAWAI R50/R50E/R50III/R100 ROM-dump",
    extensions: &["kawai12"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
