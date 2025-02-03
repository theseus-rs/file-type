use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967381: FileFormat = FileFormat {
    id: 27_967_381,
    source_type: SourceType::Wikidata,
    name: "Gravis Ultrasound patch",
    extensions: &["pat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
