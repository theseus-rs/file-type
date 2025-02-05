use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2138624: FileFormat = FileFormat {
    id: 2_138_624,
    source_type: SourceType::Wikidata,
    name: "registry file",
    extensions: &["reg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
