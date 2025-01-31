use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2001898: FileFormat = FileFormat {
    id: 2_001_898,
    puid: "wikidata/2001898",
    name: "Notation Interchange File Format",
    extensions: &["nif"],
    media_types: &["application/vnd.music-niff"],
    internal_signatures: &[],
    related_formats: &[],
};
