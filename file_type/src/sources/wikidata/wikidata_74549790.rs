use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_74549790: FileFormat = FileFormat {
    id: 74_549_790,
    source_type: SourceType::Wikidata,
    name: "Expert Witness compression Format SMART disk image",
    extensions: &["s01"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
