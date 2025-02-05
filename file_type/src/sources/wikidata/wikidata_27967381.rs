use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967381: FileFormat = FileFormat {
    id: 27_967_381,
    source_type: SourceType::Wikidata,
    name: "Gravis Ultrasound patch",
    extensions: &["pat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
