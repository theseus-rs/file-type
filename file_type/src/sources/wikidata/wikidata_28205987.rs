use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205987: FileFormat = FileFormat {
    id: 28_205_987,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive Monochrome",
    extensions: &["imm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
