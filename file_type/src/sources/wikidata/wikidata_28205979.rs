use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205979: FileFormat = FileFormat {
    id: 28_205_979,
    puid: "wikidata/28205979",
    name: "Digital Video Interactive I Color Channel",
    extensions: &["imi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
