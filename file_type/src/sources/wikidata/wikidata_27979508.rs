use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979508: FileFormat = FileFormat {
    id: 27_979_508,
    puid: "wikidata/27979508",
    name: "RIFF Palette File",
    extensions: &["pal", "pal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
