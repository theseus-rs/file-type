use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111665213: FileFormat = FileFormat {
    id: 111_665_213,
    source_type: SourceType::Wikidata,
    name: "AbiWord Collaborative File Descriptor",
    extensions: &["abicollab"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
