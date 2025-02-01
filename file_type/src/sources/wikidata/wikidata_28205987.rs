use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205987: FileFormat = FileFormat {
    id: 28_205_987,
    puid: "wikidata/28205987",
    name: "Digital Video Interactive Monochrome",
    extensions: &["imm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
