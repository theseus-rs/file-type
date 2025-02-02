use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2138624: FileFormat = FileFormat {
    id: 2_138_624,
    source_type: SourceType::Wikidata,
    name: "registry file",
    extensions: &["reg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
