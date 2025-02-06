use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111363671: FileFormat = FileFormat {
    id: 111_363_671,
    source_type: SourceType::Wikidata,
    name: "Yamaha Motif XS 'all' format",
    extensions: &["x0a"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
