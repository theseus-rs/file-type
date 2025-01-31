use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_857512: FileFormat = FileFormat {
    id: 857_512,
    puid: "wikidata/857512",
    name: "Smacker video",
    extensions: &["smk"],
    media_types: &["video/vnd.radgamettools.smacker"],
    internal_signatures: &[],
    related_formats: &[],
};
