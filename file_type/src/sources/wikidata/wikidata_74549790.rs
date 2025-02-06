use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_74549790: FileFormat = FileFormat {
    id: 74_549_790,
    source_type: SourceType::Wikidata,
    name: "Expert Witness compression Format SMART disk image",
    extensions: &["s01"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
