use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59820886: FileFormat = FileFormat {
    id: 59_820_886,
    source_type: SourceType::Wikidata,
    name: "Corel CMX Compressed",
    extensions: &["cpx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
