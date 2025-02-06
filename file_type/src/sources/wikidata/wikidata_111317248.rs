use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111317248: FileFormat = FileFormat {
    id: 111_317_248,
    source_type: SourceType::Wikidata,
    name: "Korg Triton or Trinity script file",
    extensions: &["ksc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
