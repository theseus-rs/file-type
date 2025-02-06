use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205987: FileFormat = FileFormat {
    id: 28_205_987,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive Monochrome",
    extensions: &["imm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
