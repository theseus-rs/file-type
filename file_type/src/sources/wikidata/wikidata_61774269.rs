use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61774269: FileFormat = FileFormat {
    id: 61_774_269,
    puid: "wikidata/61774269",
    name: "WavPack Binary, version 4",
    extensions: &["wv"],
    media_types: &["audio/x-wv"],
    internal_signatures: &[],
    related_formats: &[],
};
