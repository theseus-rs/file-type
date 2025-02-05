use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111356213: FileFormat = FileFormat {
    id: 111_356_213,
    source_type: SourceType::Wikidata,
    name: "Yamaha Motif (6/7/8) 'waves' format",
    extensions: &["w2w"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
