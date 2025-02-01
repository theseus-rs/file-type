use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127691413: FileFormat = FileFormat {
    id: 127_691_413,
    puid: "wikidata/127691413",
    name: "Elm file",
    extensions: &["elm"],
    media_types: &["text/x-elm"],
    internal_signatures: &[],
    related_formats: &[],
};
