use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111363686: FileFormat = FileFormat {
    id: 111_363_686,
    source_type: SourceType::Wikidata,
    name: "Yamaha Motif XS 'voices' format",
    extensions: &["x0v"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
