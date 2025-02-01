use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205982: FileFormat = FileFormat {
    id: 28_205_982,
    puid: "wikidata/28205982",
    name: "Digital Video Interactive Q Color Channel",
    extensions: &["imq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
