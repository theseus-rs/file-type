use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27960107: FileFormat = FileFormat {
    id: 27_960_107,
    puid: "wikidata/27960107",
    name: "Berkeley/IRCAM/Carl Sound Format",
    extensions: &["sf"],
    media_types: &["audio/x-ircam"],
    internal_signatures: &[],
    related_formats: &[],
};
