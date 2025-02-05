use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111363704: FileFormat = FileFormat {
    id: 111_363_704,
    source_type: SourceType::Wikidata,
    name: "Yamaha Motif XF 'all' format",
    extensions: &["x3a"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
