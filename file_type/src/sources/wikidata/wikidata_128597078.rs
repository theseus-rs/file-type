use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128597078: FileFormat = FileFormat {
    id: 128_597_078,
    puid: "wikidata/128597078",
    name: "AmbientTalk file",
    extensions: &["at"],
    media_types: &["text/x-ambienttalk"],
    internal_signatures: &[],
    related_formats: &[],
};
