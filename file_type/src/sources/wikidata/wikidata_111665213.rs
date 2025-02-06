use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111665213: FileFormat = FileFormat {
    id: 111_665_213,
    source_type: SourceType::Wikidata,
    name: "AbiWord Collaborative File Descriptor",
    extensions: &["abicollab"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
