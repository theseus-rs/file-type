use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967398: FileFormat = FileFormat {
    id: 27_967_398,
    puid: "wikidata/27967398",
    name: "AdLib Visual Composer / Roland Synthesizer song",
    extensions: &["rol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
