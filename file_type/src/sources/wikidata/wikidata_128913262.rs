use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128913262: FileFormat = FileFormat {
    id: 128_913_262,
    puid: "wikidata/128913262",
    name: "DylanLID file format",
    extensions: &["lid"],
    media_types: &["text/x-dylan-lid"],
    internal_signatures: &[],
    related_formats: &[],
};
