use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205968: FileFormat = FileFormat {
    id: 28_205_968,
    puid: "wikidata/28205968",
    name: "Digital Video Interactive Blue Channel",
    extensions: &["imb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
