use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111263191: FileFormat = FileFormat {
    id: 111_263_191,
    puid: "wikidata/111263191",
    name: "Audio CD compatible raw data",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
