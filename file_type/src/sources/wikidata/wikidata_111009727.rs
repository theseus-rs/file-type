use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111009727: FileFormat = FileFormat {
    id: 111_009_727,
    puid: "wikidata/111009727",
    name: "PrintMaster Envelope File format",
    extensions: &["env"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
