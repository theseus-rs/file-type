use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967184: FileFormat = FileFormat {
    id: 27_967_184,
    source_type: SourceType::Wikidata,
    name: "FC-M Packer module",
    extensions: &["fcm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
