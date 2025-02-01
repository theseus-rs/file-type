use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967381: FileFormat = FileFormat {
    id: 27_967_381,
    puid: "wikidata/27967381",
    name: "Gravis Ultrasound patch",
    extensions: &["pat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
