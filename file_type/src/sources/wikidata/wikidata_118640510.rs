use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118640510: FileFormat = FileFormat {
    id: 118_640_510,
    puid: "wikidata/118640510",
    name: "Anime Studio File",
    extensions: &["anme"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
