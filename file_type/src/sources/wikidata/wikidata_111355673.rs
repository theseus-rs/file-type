use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111355673: FileFormat = FileFormat {
    id: 111_355_673,
    source_type: SourceType::Wikidata,
    name: "Yamaha Motif (6/7/8) 'all' format",
    extensions: &["w2a"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
