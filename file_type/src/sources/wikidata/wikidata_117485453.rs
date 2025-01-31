use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117485453: FileFormat = FileFormat {
    id: 117_485_453,
    puid: "wikidata/117485453",
    name: "MacCaption File 2",
    extensions: &["mcc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
