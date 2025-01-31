use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121093219: FileFormat = FileFormat {
    id: 121_093_219,
    puid: "wikidata/121093219",
    name: "Punch! Home Suite PSH file",
    extensions: &["psh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
