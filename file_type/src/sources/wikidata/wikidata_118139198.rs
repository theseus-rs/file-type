use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118139198: FileFormat = FileFormat {
    id: 118_139_198,
    puid: "wikidata/118139198",
    name: "DOS Caledar File",
    extensions: &["cal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
