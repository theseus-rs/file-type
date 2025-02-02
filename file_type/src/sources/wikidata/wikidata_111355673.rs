use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111355673: FileFormat = FileFormat {
    id: 111_355_673,
    source_type: SourceType::Wikidata,
    name: "Yamaha Motif (6/7/8) 'all' format",
    extensions: &["w2a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
