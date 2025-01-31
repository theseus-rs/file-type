use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850882: FileFormat = FileFormat {
    id: 105_850_882,
    puid: "wikidata/105850882",
    name: "Kazaa Playlist",
    extensions: &["kpl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
