use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111363704: FileFormat = FileFormat {
    id: 111_363_704,
    source_type: SourceType::Wikidata,
    name: "Yamaha Motif XF 'all' format",
    extensions: &["x3a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
