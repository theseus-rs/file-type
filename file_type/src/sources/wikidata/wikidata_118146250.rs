use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118146250: FileFormat = FileFormat {
    id: 118_146_250,
    puid: "wikidata/118146250",
    name: "Stripline File",
    extensions: &["tl4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
