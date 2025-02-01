use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27966966: FileFormat = FileFormat {
    id: 27_966_966,
    puid: "wikidata/27966966",
    name: "Accolade MIDI File Format",
    extensions: &["mus"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
