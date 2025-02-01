use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116923685: FileFormat = FileFormat {
    id: 116_923_685,
    puid: "wikidata/116923685",
    name: "Super Duper Music Looper File",
    extensions: &["sdml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
