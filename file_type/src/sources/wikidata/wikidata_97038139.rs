use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_97038139: FileFormat = FileFormat {
    id: 97_038_139,
    puid: "wikidata/97038139",
    name: "LEGO Mindstorms EV3 Robot Graphics File",
    extensions: &["rfg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
