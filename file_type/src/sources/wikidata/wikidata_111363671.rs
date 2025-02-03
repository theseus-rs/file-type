use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111363671: FileFormat = FileFormat {
    id: 111_363_671,
    source_type: SourceType::Wikidata,
    name: "Yamaha Motif XS 'all' format",
    extensions: &["x0a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
