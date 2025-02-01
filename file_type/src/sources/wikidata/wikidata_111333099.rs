use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111333099: FileFormat = FileFormat {
    id: 111_333_099,
    puid: "wikidata/111333099",
    name: "Korg PA1X/PA800/PA2X samples",
    extensions: &["pcm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
