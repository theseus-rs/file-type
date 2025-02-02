use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59820886: FileFormat = FileFormat {
    id: 59_820_886,
    source_type: SourceType::Wikidata,
    name: "Corel CMX Compressed",
    extensions: &["cpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
